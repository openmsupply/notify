use chrono::NaiveDateTime;
use repository::{
    NotificationConfigKind, NotificationConfigRowRepository, NotificationConfigStatus,
};
use service::{
    notification_config::{query::NotificationConfig, recipients::get_notification_targets},
    service_provider::ServiceContext,
};

use crate::{
    alerts::{send_temperature_alert, TemperatureAlert},
    latest_temperature::{self, latest_temperature},
    parse::ColdChainPluginConfig,
    sensor_info::sensor_info,
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
    // Currently because we don't set a `next_due_time` all configurations are processed on every tick
    let configs = ctx
        .service_provider
        .notification_config_service
        .find_all_due_by_kind(ctx, NotificationConfigKind::ColdChain, current_time)
        .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;
    let num_configs = configs.len();

    log::debug!("Found {} cold chain configurations to process", num_configs);

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
            Ok(ProcessingResult::Success) => {
                log::debug!("Successfully processed coldchain config");
            }
        }
    }

    Ok(num_configs)
}

enum ProcessingResult {
    Success,
}

fn try_process_coldchain_notifications(
    ctx: &ServiceContext,
    notification_config: NotificationConfig,
    now: NaiveDateTime,
) -> Result<ProcessingResult, ColdChainError> {
    // Load the notification config
    let config = ColdChainPluginConfig::from_string(&notification_config.configuration_data)?;

    // Update the last_checked time
    NotificationConfigRowRepository::new(&ctx.connection)
        .set_last_run_by_id(&notification_config.id, now, None)
        .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;

    let high_temp_threshold: f64 = config.high_temp_threshold;
    let low_temp_threshold: f64 = config.low_temp_threshold;
    let max_age = chrono::Duration::hours(24); // TODO: Get this from config https://github.com/openmsupply/notify/issues/179

    // Put all the alerts into a vector, to simply the logic for sending alerts
    let mut alerts: Vec<TemperatureAlert> = Vec::new();

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

        let latest_temperature_row = latest_temperature(&mut connection, sensor_id.clone())
            .map_err(|e| {
                ColdChainError::InternalError(format!(
                    "Failed to get latest temperature for sensor {}: {:?}",
                    sensor_id, e
                ))
            })?;

        let sensor_status = evaluate_sensor_status(
            now,
            latest_temperature_row.clone(),
            high_temp_threshold,
            low_temp_threshold,
            max_age,
        );

        let current_temp: String = match latest_temperature_row.clone() {
            Some(row) => match row.temperature {
                Some(t) => format!("{:.2}", t), // round to 2 decimal places
                None => "Null".to_string(),
            },
            None => "Never Recorded".to_string(),
        };

        log::info!(
            "Sensor {} is currently {:?} with a temperature of {}",
            sensor_id,
            sensor_status,
            current_temp
        );

        // We need this sensor status to be unique per notification config, so we include the notification config id in the key
        // This means that the same sensor can alarm in tow different configs
        // And duplicate notifications would be sent, e.g. if your email address is in two configuration & you have the same sensor in both
        // Future deduplication efforts could be considered for this...
        let sensor_status_key = format!("sensor_status_{}_{}", sensor_id, notification_config.id);

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
                // No previous status found, so assume we were previously in the `Ok` State
                // This means we should send a notification if the sensor is not in the `Ok` state, even the first time we see it...
                log::info!(
                    "No previous status for sensor {}, assuming it used to be Ok",
                    sensor_id
                );

                let sensor_state = SensorState {
                    sensor_id: sensor_id.clone(),
                    status: SensorStatus::Ok,
                    timestamp: now,
                    temperature: None,
                };
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
                // Unable to parse the previous state, so we can't continue
                // TODO: Should we continue or just assume ok in this case???
                continue;
            }
        };

        if sensor_status == prev_sensor_status.status {
            // Status has not changed
            log::debug!(
                "Status for sensor {} has not changed since last check",
                sensor_id
            );

            // TODO Check if we need to send a reminder notification

            continue;
        }

        log::info!(
            "Status for sensor {} has changed from {:?} to {:?}",
            sensor_id,
            prev_sensor_status,
            sensor_status
        );

        // Persist the new status
        // Note: Since we only persist `new` statuses, if a sensor has always been in the ok state, it won't have a record in the plugin store
        let sensor_state = SensorState {
            sensor_id: sensor_id.clone(),
            status: sensor_status.clone(),
            timestamp: now,
            temperature: latest_temperature_row
                .map(|row| row.temperature)
                .unwrap_or(None),
        };

        let result = ctx.service_provider.plugin_service.set_value(
            ctx,
            PLUGIN_NAME.to_string(),
            sensor_status_key,
            sensor_state.to_json_string()?,
        );
        match result {
            Ok(_) => {
                log::debug!("Saved new state for sensor {}", sensor_id);
            }
            Err(e) => {
                log::error!(
                    "Failed to persist new state for sensor {}: {:?}",
                    sensor_id,
                    e
                );
                continue;
            }
        }

        // Create the a notification if correct type of notification is enabled

        // Get Sensor information from the database to use in alerts
        let sensor_row = sensor_info(&mut connection, sensor_id.clone()).map_err(|e| {
            ColdChainError::InternalError(format!(
                "Failed to get sensor info from the database {}: {:?}",
                sensor_id, e
            ))
        })?;

        let sensor_row = match sensor_row {
            Some(row) => row,
            None => {
                log::error!("No sensor info found for sensor {}", sensor_id);
                continue;
            }
        };

        let alert = match sensor_status {
            SensorStatus::HighTemp => match config.high_temp {
                true => Some(TemperatureAlert {
                    store_name: sensor_row.store_name.clone(),
                    location_name: sensor_row.location_name.clone(),
                    sensor_id: sensor_id.clone(),
                    sensor_name: sensor_row.sensor_name.clone(),
                    datetime: now,
                    temperature: current_temp,
                    alert_type: "High".to_string(),
                }),
                false => {
                    log::info!("High temp alert disabled for sensor {}", sensor_id);
                    None
                }
            },

            SensorStatus::LowTemp => match config.low_temp {
                true => Some(TemperatureAlert {
                    store_name: sensor_row.store_name.clone(),
                    location_name: sensor_row.location_name.clone(),
                    sensor_id: sensor_id.clone(),
                    sensor_name: sensor_row.sensor_name.clone(),
                    datetime: now,
                    temperature: current_temp,
                    alert_type: "Low".to_string(),
                }),
                false => {
                    log::info!("Low temp alert disabled for sensor {}", sensor_id);
                    None
                }
            },
            SensorStatus::Ok => {
                // TODO: Send OK message if enabled
                log::error!("Not sending Ok alert for sensor {}", sensor_id);
                None
            }
            SensorStatus::NoData => {
                // TODO: Send No Data message if enabled https://github.com/openmsupply/notify/issues/179
                log::error!("Not sending No Data alert for sensor {}", sensor_id);
                None // Ignore all other types for now
            }
        };

        if let Some(alert) = alert {
            alerts.push(alert);
        }
    }

    if alerts.len() == 0 {
        log::info!("No cold chain alerts to send");
        return Ok(ProcessingResult::Success);
    }
    // TODO: Suppress too many notifications in a short period of time
    // https://github.com/openmsupply/notify/issues/177

    // look up the recipients for the notification config
    let notification_targets =
        get_notification_targets(ctx, &notification_config).map_err(|e| {
            ColdChainError::InternalError(format!("Failed to get notification targets: {:?}", e))
        })?;

    for alert in alerts {
        // Send the notifications
        let result = send_temperature_alert(
            ctx,
            Some(notification_config.id.clone()),
            alert,
            notification_targets.clone(),
        );
        match result {
            Ok(_) => {
                log::info!("Successfully sent cold chain alert");
            }
            Err(e) => {
                log::error!("Failed to send cold chain alert: {:?}", e);
            }
        }
    }

    Ok(ProcessingResult::Success)
}

fn evaluate_sensor_status(
    now: NaiveDateTime,
    latest_temperature_row: Option<latest_temperature::LatestTemperatureRow>,
    high_temp_threshold: f64,
    low_temp_threshold: f64,
    max_age: chrono::Duration,
) -> SensorStatus {
    let sensor_status = match latest_temperature_row.clone() {
        None => SensorStatus::NoData, // No rows returned, means no data!
        Some(row) => match row.temperature {
            Some(t) => {
                // check if the row is too old and should be considered no data row!
                if (now - row.log_datetime) > max_age {
                    return SensorStatus::NoData;
                }
                match t {
                    t if (t > high_temp_threshold) => SensorStatus::HighTemp,
                    t if (t < low_temp_threshold) => SensorStatus::LowTemp,
                    _ => SensorStatus::Ok,
                }
            }
            None => SensorStatus::NoData, // There's a row returned but the temperature is null, so no data again!
        },
    };
    return sensor_status;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_evaluate_sensor_status() {
        let now =
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let high_temp_threshold = 8.0;
        let low_temp_threshold = 2.0;
        let max_age = chrono::Duration::hours(1);

        // Ok (High and low thresholds are within limits)
        let row = latest_temperature::LatestTemperatureRow {
            id: "1".to_string(),
            sensor_id: "1".to_string(),
            log_datetime: now,
            temperature: Some(low_temp_threshold),
        };

        let status = evaluate_sensor_status(
            now,
            Some(row),
            high_temp_threshold,
            low_temp_threshold,
            max_age,
        );
        assert_eq!(status, SensorStatus::Ok);

        let row = latest_temperature::LatestTemperatureRow {
            id: "1".to_string(),
            sensor_id: "1".to_string(),
            log_datetime: now,
            temperature: Some(high_temp_threshold),
        };

        let status = evaluate_sensor_status(
            now,
            Some(row),
            high_temp_threshold,
            low_temp_threshold,
            max_age,
        );
        assert_eq!(status, SensorStatus::Ok);

        // High Temp
        let row = latest_temperature::LatestTemperatureRow {
            id: "1".to_string(),
            sensor_id: "1".to_string(),
            log_datetime: now,
            temperature: Some(high_temp_threshold + 1.0),
        };

        let status = evaluate_sensor_status(
            now,
            Some(row),
            high_temp_threshold,
            low_temp_threshold,
            max_age,
        );
        assert_eq!(status, SensorStatus::HighTemp);

        // Low Temp

        let row = latest_temperature::LatestTemperatureRow {
            id: "1".to_string(),
            sensor_id: "1".to_string(),
            log_datetime: now,
            temperature: Some(low_temp_threshold - 1.0),
        };

        let status = evaluate_sensor_status(
            now,
            Some(row),
            high_temp_threshold,
            low_temp_threshold,
            max_age,
        );
        assert_eq!(status, SensorStatus::LowTemp);

        // No Data (no Row)

        let status =
            evaluate_sensor_status(now, None, high_temp_threshold, low_temp_threshold, max_age);
        assert_eq!(status, SensorStatus::NoData);

        // No Data (row with null temp)
        let row = latest_temperature::LatestTemperatureRow {
            id: "1".to_string(),
            sensor_id: "1".to_string(),
            log_datetime: now,
            temperature: None,
        };

        let status = evaluate_sensor_status(
            now,
            Some(row),
            high_temp_threshold,
            low_temp_threshold,
            max_age,
        );
        assert_eq!(status, SensorStatus::NoData);

        // No Data (row too old)

        let row = latest_temperature::LatestTemperatureRow {
            id: "1".to_string(),
            sensor_id: "1".to_string(),
            log_datetime: now - chrono::Duration::hours(2),
            temperature: Some(low_temp_threshold),
        };

        let status = evaluate_sensor_status(
            now,
            Some(row),
            high_temp_threshold,
            low_temp_threshold,
            max_age,
        );
        // TODO: Old Data Logic https://github.com/openmsupply/notify/issues/179
        assert_eq!(status, SensorStatus::NoData);
    }
}
