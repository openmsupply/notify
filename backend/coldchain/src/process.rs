use chrono::{DateTime, Local, NaiveDateTime, Utc};
use repository::{
    NotificationConfigKind, NotificationConfigRowRepository, NotificationConfigStatus,
};
use service::{
    notification_config::{query::NotificationConfig, recipients::get_notification_targets},
    service_provider::ServiceContext,
};

use crate::{
    alerts::{queue_temperature_alert, AlertType, ColdchainAlert},
    latest_temperature::{self, latest_temperature},
    parse::ColdChainPluginConfig,
    sensor_info::{sensor_info, SensorInfoRow},
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

    // We compare the returned data from the database to the local time as it's recorded in localtime
    // BUG: This means that during daily daylight savings time changes, the notifications might be incorrect (from last year, or we might trigger phantom no-data issues)
    // This needs to be fixed in the datasource before we can improve the logic here...
    let local_tz = Local::now().offset().clone(); // clone is needed to avoid lifetime issues
    let now_local: NaiveDateTime = DateTime::<Local>::from_utc(now, local_tz).naive_local();
    log::info!(
        "Processing cold chain config ({}) @ {} local time",
        notification_config.title,
        now_local
    );

    // Update the last_checked time
    NotificationConfigRowRepository::new(&ctx.connection)
        .set_last_run_by_id(&notification_config.id, now, None)
        .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;

    // Put all the alerts into a vector, to simply the logic for sending alerts
    let mut alerts: Vec<ColdchainAlert> = Vec::new();

    // Loop through checking the current status for each sensor
    for sensor_id in config.sensor_ids.clone() {
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

        // We need this sensor status to be unique per notification config, so we include the notification config id in the key
        // This means that the same sensor can alarm in two different configs
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
                    last_data_localtime: now_local,
                    status_start_utc: now,
                    temperature: None,
                    last_notification_utc: None,
                    reminder_number: 0,
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
                continue;
            }
        };

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

        let result = try_process_sensor_notification(
            &config,
            prev_sensor_status.clone(),
            sensor_row,
            now_local,
            latest_temperature_row,
        );
        let (sensor_state, alert) = match result {
            Ok((state, alert)) => (state, alert),
            Err(e) => {
                log::error!(
                    "Failed to process sensor notification for sensor {}: {:?}",
                    sensor_id,
                    e
                );
                // Unable to process the sensor notification, so we can't continue
                continue;
            }
        };

        // if we have an updated state, persist it...
        if sensor_state.status != prev_sensor_status.status {
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
                }
            };
        }

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
    let notification_targets = get_notification_targets(
        ctx,
        &notification_config,
        serde_json::Value::Null,
    )
    .map_err(|e| {
        ColdChainError::InternalError(format!("Failed to get notification targets: {:?}", e))
    })?;

    for alert in alerts {
        // Send the notifications
        let result = queue_temperature_alert(
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

pub fn try_process_sensor_notification(
    config: &ColdChainPluginConfig,
    prev_sensor_state: SensorState,
    sensor_row: SensorInfoRow,
    now_local: NaiveDateTime,
    latest_temperature_row: Option<latest_temperature::LatestTemperatureRow>,
) -> Result<(SensorState, Option<ColdchainAlert>), ColdChainError> {
    let curr_sensor_status = evaluate_sensor_status(
        now_local,
        latest_temperature_row.clone(),
        config.high_temp_threshold,
        config.low_temp_threshold,
        config.no_data_duration(),
    );

    log::info!(
        "Sensor {} is currently {:?}",
        sensor_row.id,
        curr_sensor_status
    );

    let mut reminder_number = 0;
    let mut reminder_timestamp = None;
    let mut status_start_utc = prev_sensor_state.status_start_utc;

    if curr_sensor_status == prev_sensor_state.status {
        // Status has not changed
        log::debug!(
            "Status for sensor {} has not changed since last check ({:?})",
            sensor_row.id,
            curr_sensor_status
        );

        if curr_sensor_status == SensorStatus::Ok {
            // If the sensor is ok, we don't need to send a reminder
            return Ok((prev_sensor_state, None));
        }

        // Check if if a reminder is due
        let last_alert_timestamp = match prev_sensor_state.last_notification_utc {
            Some(t) => t,
            None => prev_sensor_state.status_start_utc,
        };

        if last_alert_timestamp + config.reminder_duration() > Utc::now().naive_utc() {
            // It's not time to send a reminder yet
            log::debug!(
                "Not sending reminder for sensor {} which has been in state {:?} since {} (utc)",
                sensor_row.id,
                curr_sensor_status,
                prev_sensor_state.status_start_utc
            );
            // return the previous state, and no alert
            return Ok((prev_sensor_state, None));
        }

        // It's time to send a reminder!
        log::info!(
            "A reminder is due for {} : {:?}",
            sensor_row.id,
            curr_sensor_status
        );
        reminder_number = prev_sensor_state.reminder_number + 1;
        reminder_timestamp = Some(Utc::now().naive_utc());
    } else {
        log::info!(
            "Status for sensor {} has changed from {:?} to {:?}",
            sensor_row.id,
            prev_sensor_state.status,
            curr_sensor_status
        );
        status_start_utc = Utc::now().naive_utc();
    }

    let last_data_localtime: NaiveDateTime = latest_temperature_row
        .clone()
        .map(|row| row.log_datetime)
        .unwrap_or_default();

    let data_age: String = match latest_temperature_row.clone() {
        Some(row) => format!(
            "{} minutes",
            (Local::now().naive_local() - row.log_datetime)
                .num_minutes()
                .to_string() // TODO: Improve this to show the age in hours/days/weeks/months/years? Ideally it would be translatable?
        ),
        None => "?? minutes".to_string(),
    };

    let current_temp: String = match latest_temperature_row.clone() {
        Some(row) => match row.temperature {
            Some(t) => format!("{:.2}", t), // round to 2 decimal places
            None => "Null".to_string(),
        },
        None => "Never Recorded".to_string(),
    };

    // Calculate the new sensor state
    let sensor_state = SensorState {
        sensor_id: sensor_row.id.clone(),
        status: curr_sensor_status.clone(),
        last_data_localtime: latest_temperature_row
            .clone()
            .map(|row| row.log_datetime)
            .unwrap_or_default(),
        temperature: latest_temperature_row
            .map(|row| row.temperature)
            .unwrap_or(None),
        status_start_utc,
        last_notification_utc: reminder_timestamp,
        reminder_number,
    };

    let alert = match curr_sensor_status {
        SensorStatus::HighTemp => match config.high_temp {
            true => Some(ColdchainAlert {
                store_name: sensor_row.store_name.clone(),
                location_name: sensor_row.location_name.clone(),
                sensor_id: sensor_row.id.clone(),
                sensor_name: sensor_row.sensor_name.clone(),
                last_data_time: sensor_state.last_data_localtime,
                data_age,
                temperature: current_temp,
                alert_type: AlertType::High,
                reminder_number: None,
            }),
            false => {
                log::info!("High temp alert disabled for sensor {}", sensor_row.id);
                None
            }
        },

        SensorStatus::LowTemp => match config.low_temp {
            true => Some(ColdchainAlert {
                store_name: sensor_row.store_name.clone(),
                location_name: sensor_row.location_name.clone(),
                sensor_id: sensor_row.id.clone(),
                sensor_name: sensor_row.sensor_name.clone(),
                last_data_time: last_data_localtime,
                data_age,
                temperature: current_temp,
                alert_type: AlertType::Low,
                reminder_number: None,
            }),
            false => {
                log::info!("Low temp alert disabled for sensor {}", sensor_row.id);
                None
            }
        },
        SensorStatus::Ok => match config.confirm_ok {
            true => Some(ColdchainAlert {
                store_name: sensor_row.store_name.clone(),
                location_name: sensor_row.location_name.clone(),
                sensor_id: sensor_row.id.clone(),
                sensor_name: sensor_row.sensor_name.clone(),
                last_data_time: last_data_localtime,
                data_age,
                temperature: current_temp,
                alert_type: AlertType::Ok,
                reminder_number: None,
            }),
            false => {
                log::info!("Confirm Ok alert disabled for sensor {}", sensor_row.id);
                None
            }
        },
        SensorStatus::NoData => match config.no_data {
            true => Some(ColdchainAlert {
                store_name: sensor_row.store_name.clone(),
                location_name: sensor_row.location_name.clone(),
                sensor_id: sensor_row.id.clone(),
                sensor_name: sensor_row.sensor_name.clone(),
                last_data_time: last_data_localtime,
                data_age,
                temperature: current_temp,
                alert_type: AlertType::NoData,
                reminder_number: None,
            }),
            false => {
                log::info!("No data alert disabled for sensor {}", sensor_row.id);
                None
            }
        },
    };

    Ok((sensor_state, alert))
}

pub fn evaluate_sensor_status(
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
