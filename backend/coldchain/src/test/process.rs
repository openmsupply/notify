use chrono::{NaiveDateTime, Utc};

use crate::{
    alerts::AlertType,
    latest_temperature::LatestTemperatureRow,
    parse::ColdChainPluginConfig,
    process::{evaluate_sensor_status, try_process_sensor_notification},
    sensor_info::SensorInfoRow,
    sensor_state::{SensorState, SensorStatus},
};

#[test]
fn test_evaluate_sensor_status() {
    let now = NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let high_temp_threshold = 8.0;
    let low_temp_threshold = 2.0;
    let max_age = chrono::Duration::hours(1);

    // Ok (High and low thresholds are within limits)
    let row = LatestTemperatureRow {
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

    let row = LatestTemperatureRow {
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
    let row = LatestTemperatureRow {
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

    let row = LatestTemperatureRow {
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
    let row = LatestTemperatureRow {
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

    let row = LatestTemperatureRow {
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

#[test]
fn test_try_process_sensor_notification_prev_ok() {
    /*
       Config with all alerts enabled
       1 Hour Reminders
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: true,
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    // Sensor Data
    let sensor_row = SensorInfoRow {
        id: "1".to_string(),
        sensor_name: "Sensor 1".to_string(),
        location_name: "Location 1".to_string(),
        store_name: "Store 1".to_string(),
        store_id: String::new(),
        batterylevel: Some(90.0),
    };

    // Time Now (Local Time)
    let now_local =
        NaiveDateTime::parse_from_str("2020-01-01T00:01:00", "%Y-%m-%dT%H:%M:%S").unwrap();

    // Previous Sensor State Ok, 1 Minute Ago
    let prev_sensor_state_ok_1min = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::Ok,
        timestamp_localtime: now_local - chrono::Duration::minutes(1),
        temperature: Some(5.5),
        status_start_utc: Utc::now().naive_utc() - chrono::Duration::minutes(1),
        last_notification_utc: None,
        reminder_number: 0,
    };

    // Previous Sensor State Ok > no_data duration ago
    let prev_sensor_state_ok_now_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::Ok,
        timestamp_localtime: now_local - config.no_data_duration() - chrono::Duration::minutes(1),
        temperature: Some(5.5),
        status_start_utc: Utc::now().naive_utc()
            - config.no_data_duration()
            - chrono::Duration::minutes(1),
        last_notification_utc: None,
        reminder_number: 0,
    };

    /*
        Test 1: Was Ok 1 Minute ago, Still Ok Now: No Alert
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(5.5), // Within limits
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_ok_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::Ok);
    assert_eq!(alert.is_none(), true);

    /*
        Test 2: Was Ok 1 Minute ago, Now High Temp -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_ok_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::High);

    /*
        Test 3: Was Ok 1 Minute ago, Now Low Temp -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.low_temp_threshold - 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_ok_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::LowTemp);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::Low);

    /*
        Test 4: Was Ok 1 Minute ago, Now No Data -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local - config.no_data_duration() - chrono::Duration::minutes(1),
        temperature: Some(5.5),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_ok_now_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::NoData);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::NoData);
}

#[test]
fn test_try_process_sensor_notification_prev_high() {
    /*
       Config with all alerts enabled
       1 Hour Reminders
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: true,
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    // Sensor Data
    let sensor_row = SensorInfoRow {
        id: "1".to_string(),
        sensor_name: "Sensor 1".to_string(),
        location_name: "Location 1".to_string(),
        store_name: "Store 1".to_string(),
        store_id: String::new(),
        batterylevel: Some(90.0),
    };

    // Time Now (Local Time)
    let now_local =
        NaiveDateTime::parse_from_str("2020-01-01T00:01:00", "%Y-%m-%dT%H:%M:%S").unwrap();

    // Previous Sensor State High, 1 Minute Ago
    let prev_sensor_state_high_1min = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::HighTemp,
        timestamp_localtime: now_local - chrono::Duration::minutes(1),
        temperature: Some(config.high_temp_threshold + 1.0),
        status_start_utc: Utc::now().naive_utc() - chrono::Duration::minutes(1),
        last_notification_utc: None,
        reminder_number: 0,
    };

    // Previous Sensor State High > no_data duration ago
    let prev_sensor_state_high_now_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::HighTemp,
        timestamp_localtime: now_local - config.no_data_duration() - chrono::Duration::minutes(1),
        temperature: Some(config.high_temp_threshold + 1.0),
        status_start_utc: Utc::now().naive_utc()
            - config.no_data_duration()
            - chrono::Duration::minutes(1),
        last_notification_utc: None,
        reminder_number: 0,
    };

    /*
        Test 1: Was High 1 Minute ago, Ok Now: Ok Alert
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(5.5), // Within limits
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_high_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::Ok);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::Ok);

    /*
        Test 2: Was High 1 Minute ago, Now High Temp -> No Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_high_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert_eq!(alert.is_none(), true);

    /*
        Test 3: Was High 1 Minute ago, Now Low Temp -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.low_temp_threshold - 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_high_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::LowTemp);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::Low);

    /*
        Test 4: Was High 1 Minute ago, Now No Data -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local - config.no_data_duration() - chrono::Duration::minutes(1),
        temperature: Some(5.5),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_high_now_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::NoData);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::NoData);
}

#[test]
fn test_try_process_sensor_notification_prev_low() {
    /*
       Config with all alerts enabled
       1 Hour Reminders
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: true,
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    // Sensor Data
    let sensor_row = SensorInfoRow {
        id: "1".to_string(),
        sensor_name: "Sensor 1".to_string(),
        location_name: "Location 1".to_string(),
        store_name: "Store 1".to_string(),
        store_id: String::new(),
        batterylevel: Some(90.0),
    };

    // Time Now (Local Time)
    let now_local =
        NaiveDateTime::parse_from_str("2020-01-01T00:01:00", "%Y-%m-%dT%H:%M:%S").unwrap();

    // Previous Sensor State Low, 1 Minute Ago
    let prev_sensor_state_low_1min = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::LowTemp,
        timestamp_localtime: now_local - chrono::Duration::minutes(1),
        temperature: Some(config.low_temp_threshold - 1.0),
        status_start_utc: Utc::now().naive_utc() - chrono::Duration::minutes(1),
        last_notification_utc: None,
        reminder_number: 0,
    };

    // Previous Sensor State Low > no_data duration ago
    let prev_sensor_state_low_now_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::LowTemp,
        timestamp_localtime: now_local - config.no_data_duration() - chrono::Duration::minutes(1),
        temperature: Some(config.low_temp_threshold - 1.0),
        status_start_utc: Utc::now().naive_utc()
            - config.no_data_duration()
            - chrono::Duration::minutes(1),
        last_notification_utc: None,
        reminder_number: 0,
    };

    /*
        Test 1: Was Low 1 Minute ago, Ok Now: Ok Alert
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(5.5), // Within limits
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_low_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::Ok);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::Ok);

    /*
        Test 2: Was Low 1 Minute ago, Now High Temp -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_low_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::High);

    /*
        Test 3: Was Low 1 Minute ago, Now Low Temp -> No Alert
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.low_temp_threshold - 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_low_1min.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::LowTemp);
    assert_eq!(alert.is_none(), true);

    /*
        Test 4: Was Low 1 Minute ago, Now No Data -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local - config.no_data_duration() - chrono::Duration::minutes(1),
        temperature: Some(5.5),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_low_now_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::NoData);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::NoData);
}

#[test]
fn test_try_process_sensor_notification_prev_no_data() {
    /*
       Config with all alerts enabled
       1 Hour Reminders
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: true,
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    // Sensor Data
    let sensor_row = SensorInfoRow {
        id: "1".to_string(),
        sensor_name: "Sensor 1".to_string(),
        location_name: "Location 1".to_string(),
        store_name: "Store 1".to_string(),
        store_id: String::new(),
        batterylevel: Some(90.0),
    };

    // Time Now (Local Time)
    let now_local =
        NaiveDateTime::parse_from_str("2020-01-01T00:01:00", "%Y-%m-%dT%H:%M:%S").unwrap();

    let last_data_timestamp = now_local - config.no_data_duration() - chrono::Duration::minutes(2);

    // Previous Sensor State No Data 1 minute ago
    let prev_sensor_state_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::NoData,
        timestamp_localtime: last_data_timestamp.clone(),
        temperature: Some(5.5),
        status_start_utc: Utc::now().naive_utc() - chrono::Duration::minutes(1),
        last_notification_utc: None,
        reminder_number: 0,
    };

    /*
        Test 1: Was No Data 1 Minute ago, Ok Now: Ok Alert
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(5.5), // Within limits
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::Ok);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::Ok);

    /*
        Test 2: Was No Data 1 Minute ago, Now High Temp -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::High);

    /*
        Test 3: Was No Data 1 Minute ago, Now Low Temp -> Alert!
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.low_temp_threshold - 1.0),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::LowTemp);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::Low);

    /*
        Test 4: Was No Data, Now Still No Data -> No Alert
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: last_data_timestamp,
        temperature: Some(5.5),
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();

    assert_eq!(sensor_state.status, SensorStatus::NoData);
    assert_eq!(alert.is_none(), true);
}

#[test]
fn test_try_process_sensor_notification_no_data_reminder() {
    /*
       Config with all alerts enabled
       1 Hour Reminders
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: true,
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    // Sensor Data
    let sensor_row = SensorInfoRow {
        id: "1".to_string(),
        sensor_name: "Sensor 1".to_string(),
        location_name: "Location 1".to_string(),
        store_name: "Store 1".to_string(),
        store_id: String::new(),
        batterylevel: Some(90.0),
    };

    // Time Now (Local Time)
    let now_local =
        NaiveDateTime::parse_from_str("2020-01-01T00:01:00", "%Y-%m-%dT%H:%M:%S").unwrap();

    // Assume we've gone no data for 1 hour (no data duration + reminder duration)
    let last_data_timestamp = now_local
        - config.no_data_duration()
        - config.reminder_duration()
        - chrono::Duration::minutes(1);

    // Previous Sensor State No Data 1 minute ago
    let prev_sensor_state_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::NoData,
        timestamp_localtime: last_data_timestamp.clone(),
        temperature: Some(5.5),
        status_start_utc: Utc::now().naive_utc()
            - config.reminder_duration()
            - chrono::Duration::minutes(1),
        last_notification_utc: Some(Utc::now().naive_utc() - config.no_data_duration()),
        reminder_number: 0,
    };

    /*
        Test 1: Has been No Data for 1 hour (Reminder Duration) but still no data -> Send a reminder
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: last_data_timestamp,
        temperature: Some(5.5), // Within limits
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::NoData);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::NoData);
    assert_eq!(sensor_state.reminder_number, 1);

    // If reminders are turned off, we shouldn't get a reminder...
    /*
       Config with all alerts enabled
       NO REMINDERS
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: false, // Reminders disabled!
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    /*
        Test 2: Has been No Data for 1 hour (e.g. Reminder Duration) and still no data BUT reminders are turned off -> Don't send a reminder
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: last_data_timestamp,
        temperature: Some(5.5), // Within limits
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::NoData);
    assert_eq!(alert.is_none(), true);
}

#[test]
fn test_try_process_sensor_notification_high_temp_reminder() {
    /*
       Config with all alerts enabled
       1 Hour Reminders
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: true,
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    // Sensor Data
    let sensor_row = SensorInfoRow {
        id: "1".to_string(),
        sensor_name: "Sensor 1".to_string(),
        location_name: "Location 1".to_string(),
        store_name: "Store 1".to_string(),
        store_id: String::new(),
        batterylevel: Some(90.0),
    };

    // Time Now (Local Time)
    let now_local =
        NaiveDateTime::parse_from_str("2020-01-01T00:01:00", "%Y-%m-%dT%H:%M:%S").unwrap();

    // Assume we've been High Temperature for 1 hour (reminder duration)
    let prev_sensor_state_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::HighTemp,
        timestamp_localtime: now_local - config.reminder_duration() - chrono::Duration::minutes(1),
        temperature: Some(5.5),
        status_start_utc: Utc::now().naive_utc()
            - config.reminder_duration()
            - chrono::Duration::minutes(1),
        last_notification_utc: Some(
            Utc::now().naive_utc() - config.reminder_duration() - chrono::Duration::minutes(1),
        ),
        reminder_number: 0,
    };

    /*
        Test 1: Has been High for 1 hour (Reminder Duration) and we're still High Temp -> Send a reminder
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0), // High Temp
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert!(match sensor_state.last_notification_utc {
        Some(t) => t > Utc::now().naive_utc() - chrono::Duration::minutes(1), // The reminder notification created within the last minute
        None => false,
    });
    assert_eq!(sensor_state.reminder_number, 1);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::High);
    assert_eq!(alert.reminder_number, 1);

    /*
        Test 2: Has been High for more than 1 hour (Reminder Duration) but less than 2 -> Since we already sent a reminder don't send another, yet.
    */

    let prev_sensor_state_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::HighTemp,
        timestamp_localtime: now_local - config.reminder_duration() - chrono::Duration::minutes(30),
        temperature: Some(5.5),
        status_start_utc: Utc::now().naive_utc()
            - config.reminder_duration()
            - chrono::Duration::minutes(30),
        last_notification_utc: Some(
            Utc::now().naive_utc() - config.reminder_duration() + chrono::Duration::minutes(1),
        ),
        reminder_number: 1,
    };

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0), // High Temp
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert_eq!(alert.is_none(), true);

    /*
        Test 3: Has been High for more than 2 hours (Reminder Duration 2) and we're still High Temp -> Send a second reminder
    */

    // Assume we've been High Temperature for more than 2 hours (reminder duration *2)
    let prev_sensor_state_no_data = SensorState {
        sensor_id: "1".to_string(),
        status: SensorStatus::HighTemp,
        timestamp_localtime: now_local
            - config.reminder_duration() * 2
            - chrono::Duration::minutes(1),
        temperature: Some(5.5),
        status_start_utc: Utc::now().naive_utc()
            - config.reminder_duration() * 2
            - chrono::Duration::minutes(1),
        last_notification_utc: Some(
            Utc::now().naive_utc() - config.reminder_duration() - chrono::Duration::minutes(1),
        ),
        reminder_number: 1,
    };

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0), // High Temp
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert_eq!(alert.is_some(), true);
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, AlertType::High);
    assert_eq!(sensor_state.reminder_number, 2);
    assert_eq!(alert.reminder_number, 2);

    // If reminders are turned off, we shouldn't get a reminder...
    /*
       Config with all alerts enabled
       NO REMINDERS
       1 Hour Timeout for No Data
    */

    let config = ColdChainPluginConfig {
        sensor_ids: vec!["1".to_string()],
        high_temp: true,
        high_temp_threshold: 8.0,
        low_temp: true,
        low_temp_threshold: 2.0,
        no_data: true,
        confirm_ok: true,
        no_data_interval: 1,
        no_data_interval_units: service::notification_config::intervals::IntervalUnits::Hours,
        remind: false, // Reminders disabled!
        reminder_interval: 1,
        reminder_units: service::notification_config::intervals::IntervalUnits::Hours,
    };

    /*
        Test 3: Has been No Data for 1 hour (e.g. Reminder Duration) and still no data BUT reminders are turned off -> Don't send a reminder
    */

    let latest_temperature_row = Some(LatestTemperatureRow {
        id: "1".to_string(),
        sensor_id: "1".to_string(),
        log_datetime: now_local,
        temperature: Some(config.high_temp_threshold + 1.0), // High Temp
    });

    let (sensor_state, alert) = try_process_sensor_notification(
        &config,
        prev_sensor_state_no_data.clone(),
        sensor_row.clone(),
        now_local,
        latest_temperature_row,
    )
    .unwrap();
    assert_eq!(sensor_state.status, SensorStatus::HighTemp);
    assert_eq!(alert.is_none(), true);
}
