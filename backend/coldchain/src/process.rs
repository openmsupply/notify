use chrono::NaiveDateTime;
use repository::{
    NotificationConfigKind, NotificationConfigRow, NotificationConfigRowRepository,
    NotificationConfigStatus, NotificationType,
};
use service::{
    notification::enqueue::{create_notification_events, NotificationContext, NotificationTarget},
    service_provider::ServiceContext,
};

use crate::{
    latest_temperatures::latest_temperatures,
    parse::ColdChainPluginConfig,
    sensor_state::{SensorState, SensorStatus},
    ColdChainError, PLUGIN_NAME,
};

pub fn process_coldchain_alerts(
    ctx: &ServiceContext,
    current_time: NaiveDateTime,
) -> Result<usize, ColdChainError> {
    log::info!(
        "Processing cold_chain configurations due at {}",
        current_time
    );

    // Check if any cold chain configurations are due to be processed
    let configs = ctx
        .service_provider
        .notification_config_service
        .find_all_due_by_kind(ctx, NotificationConfigKind::ColdChain, current_time)
        .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;
    let num_configs = configs.len();

    println!("Found {} cold chain configurations to process", num_configs);

    for config in configs {
        if config.status != NotificationConfigStatus::Enabled {
            log::info!(
                "Skipping cold chain config {} ({}) as it is not enabled",
                config.id,
                config.title
            );
            continue;
        }
        let result = try_process_coldchain_notifications(ctx, config, current_time);
        match result {
            Err(e) => {
                log::error!("{:?}", e);
            }
            Ok(ProcessingResult::Skipped(message)) => {
                log::info!("{}", message);
            }
            Ok(ProcessingResult::Success) => {
                log::debug!("Successfully processed coldchain config");
            }
        }
    }

    Ok(num_configs)
}

enum ProcessingResult {
    Success,
    Skipped(String),
}

fn try_process_coldchain_notifications(
    ctx: &ServiceContext,
    notification_config: NotificationConfigRow,
    now: NaiveDateTime,
) -> Result<ProcessingResult, ColdChainError> {
    // Load the notification config
    let config = ColdChainPluginConfig::from_string(&notification_config.configuration_data)?;

    // Update the last_checked time
    NotificationConfigRowRepository::new(&ctx.connection)
        .update_one(&NotificationConfigRow {
            last_run_datetime: Some(now),
            ..notification_config.clone()
        })
        .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;

    let high_temp_threshold: f64 = 22.0; // TODO: Get this from config
    let low_temp_threshold: f64 = 20.0; // TODO: Get this from config

    // Loop through checking the current status for each sensor
    for sensor_id in config.sensor_ids {
        // Get the latest temperature for the sensor
        let mut connection = ctx
            .service_provider
            .datasource_service
            .get_connection_pool()
            .pool
            .get()
            .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;

        let latest_temperature_row = latest_temperatures(&mut connection, vec![sensor_id.clone()])
            .map_err(|e| {
                ColdChainError::InternalError(format!(
                    "Failed to get latest temperature for sensor {}: {:?}",
                    sensor_id, e
                ))
            })?;
        // TODO(optimise) might be more efficient to get all latest temperatures in one query?

        let sensor_status = match latest_temperature_row.len() {
            0 => SensorStatus::NoData, // No rows returned, means no data!
            _ => match latest_temperature_row[0].temperature {
                Some(t) => match t {
                    t if (t > high_temp_threshold) => SensorStatus::HighTemp,
                    t if (t < low_temp_threshold) => SensorStatus::LowTemp,
                    _ => SensorStatus::Ok,
                },
                None => SensorStatus::NoData, // There's a row returned but the temperature is null, so no data again!
            },
        };

        let current_temp = match latest_temperature_row.get(0) {
            Some(t) => t.temperature.unwrap_or(0.0),
            None => 0.0,
        }; // This current temp is just used for the log message, so we just set it to 0.0 if there's no data which is ok I think...

        log::info!(
            "Sensor {} is currently {:?} with a temperature of {}",
            sensor_id,
            sensor_status,
            current_temp
        );

        let sensor_status_key = format!("sensor_status_{}", sensor_id);

        // Check if the status has changed since the last time we checked
        let prev_sensor_status = ctx
            .service_provider
            .plugin_service
            .get_value(ctx, PLUGIN_NAME.to_string(), sensor_status_key.clone())
            .map_err(|e| {
                ColdChainError::InternalError(format!(
                    "Failed to get previous state for sensor {}: {:?}",
                    sensor_id, e
                ))
            })?;

        let prev_sensor_status = match prev_sensor_status {
            Some(s) => SensorState::from_string(&s),
            None => {
                // No previous status, so set it and skip
                log::info!(
                    "No previous status for sensor {}, so setting to {:?}",
                    sensor_id,
                    sensor_status
                );

                let sensor_state = SensorState {
                    sensor_id: sensor_id.clone(),
                    status: sensor_status.clone(),
                    timestamp: now,
                };

                let state_json = sensor_state.to_json_string()?;

                let result = ctx.service_provider.plugin_service.set_value(
                    ctx,
                    PLUGIN_NAME.to_string(),
                    sensor_status_key,
                    state_json,
                );
                match result {
                    Ok(_) => {
                        log::debug!("Successfully set status for sensor {}", sensor_id);
                    }
                    Err(e) => {
                        log::error!("Failed to set status for sensor {}: {:?}", sensor_id, e);
                    }
                }
                Ok(sensor_state)
            }
        };

        let prev_sensor_status = match prev_sensor_status {
            Ok(s) => s,
            Err(e) => {
                log::error!(
                    "Failed to parse previous state for sensor {}: {:?}",
                    sensor_id,
                    e
                );
                continue;
            }
        };

        if sensor_status == prev_sensor_status.status {
            // Status has not changed, so skip
            log::info!(
                "Status for sensor {} has not changed since last check",
                sensor_id
            ); // TODO: change to debug, once we're confident in the logic!

            // TODO Check if we need to send a reminder notification

            continue;
        }

        log::info!(
            "Status for sensor {} has changed from {:?} to {:?}",
            sensor_id,
            prev_sensor_status,
            sensor_status
        );
        //TODO: Notifications!!!

        // create_notification_events(ctx, Some(scheduled_notification.id), notification)
        //     .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;
    }

    Ok(ProcessingResult::Success)
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use service::test_utils::get_test_settings;

    use service::service_provider::ServiceProvider;

    use super::*;
}
