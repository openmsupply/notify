/* This file contains a Struct to store sensor state, Ok, HighTemp, LowTemp, or NoData along with a timestamp */

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::ColdChainError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct SensorState {
    pub sensor_id: String,
    pub status: SensorStatus,
    #[serde(alias = "timestamp")]
    pub timestamp_localtime: NaiveDateTime,
    pub temperature: Option<f64>,
    #[serde(default)]
    pub status_start_utc: NaiveDateTime,
    #[serde(default)]
    pub last_notification_utc: Option<NaiveDateTime>,
    #[serde(default)]
    pub reminder_number: usize,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum SensorStatus {
    #[default]
    Ok,
    LowTemp,
    HighTemp,
    NoData,
}

impl SensorState {
    pub fn from_string(json_string: &str) -> Result<Self, ColdChainError> {
        let state: SensorState = serde_json::from_str(json_string)
            .map_err(|e| ColdChainError::UnableToParseConfig(format!("{:?}", e)))?;

        Ok(state)
    }

    pub fn to_json_string(&self) -> Result<String, ColdChainError> {
        let json_string = serde_json::to_string(&self)
            .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;

        Ok(json_string)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_status_ok_with_status_start_utc() {
        let example1 = r#"{ "sensor_id": "1234", "status": "Ok", "timestamp_localtime": "2020-01-01T00:00:00", "status_start_utc": "2019-09-01T00:00:00" }"#;
        let result = SensorState::from_string(example1);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.sensor_id, "1234");
        assert_eq!(state.status, SensorStatus::Ok);
        assert_eq!(
            state.timestamp_localtime,
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );
        assert_eq!(
            state.status_start_utc,
            NaiveDateTime::parse_from_str("2019-09-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );

        // Now save as json
        let json_string = state.to_json_string().unwrap();
        let result = SensorState::from_string(&json_string).unwrap();
        assert_eq!(state, result);
    }

    #[test]
    fn test_parse_status_ok() {
        let example1 =
            r#"{ "sensor_id": "1234", "status": "Ok", "timestamp": "2020-01-01T00:00:00" }"#;
        let result = SensorState::from_string(example1);
        println!("{:?}", result);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.sensor_id, "1234");
        assert_eq!(state.status, SensorStatus::Ok);
        assert_eq!(
            state.timestamp_localtime,
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );

        // Now save as json
        let json_string = state.to_json_string().unwrap();
        let result = SensorState::from_string(&json_string).unwrap();
        assert_eq!(state, result);
    }

    #[test]
    fn test_parse_status_low_temp() {
        let example1 =
            r#"{ "sensor_id": "1234", "status": "LowTemp", "timestamp": "2020-01-01T00:00:00" }"#;
        let result = SensorState::from_string(example1);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.sensor_id, "1234");
        assert_eq!(state.status, SensorStatus::LowTemp);
        assert_eq!(
            state.timestamp_localtime,
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );

        // Now save as json
        let json_string = state.to_json_string().unwrap();
        let result = SensorState::from_string(&json_string).unwrap();
        assert_eq!(state, result);
    }

    #[test]
    fn test_parse_status_high_temp() {
        let example1 =
            r#"{ "sensor_id": "1234", "status": "HighTemp", "timestamp": "2020-01-01T00:00:00" }"#;
        let result = SensorState::from_string(example1);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.sensor_id, "1234");
        assert_eq!(state.status, SensorStatus::HighTemp);
        assert_eq!(
            state.timestamp_localtime,
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );

        // Now save as json
        let json_string = state.to_json_string().unwrap();
        let result = SensorState::from_string(&json_string).unwrap();
        assert_eq!(state, result);
    }

    #[test]
    fn test_parse_status_no_data() {
        let example1 =
            r#"{ "sensor_id": "1234", "status": "NoData", "timestamp": "2020-01-01T00:00:00" }"#;
        let result = SensorState::from_string(example1);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.sensor_id, "1234");
        assert_eq!(state.status, SensorStatus::NoData);
        assert_eq!(
            state.timestamp_localtime,
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );

        // Now save as json
        let json_string = state.to_json_string().unwrap();
        let result = SensorState::from_string(&json_string).unwrap();
        assert_eq!(state, result);
    }

    #[test]
    fn test_parse_status_no_data_reminder() {
        let example1 = r#"{ "sensor_id": "1234", "status": "NoData", "timestamp_localtime": "2020-01-01T00:00:00", "last_notification_utc": "2020-01-01T01:00:00", "status_start_utc": "2020-01-01T01:00:00", "reminder_number":1  }"#;
        let result = SensorState::from_string(example1);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.sensor_id, "1234");
        assert_eq!(state.status, SensorStatus::NoData);
        assert_eq!(
            state.timestamp_localtime,
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );
        assert_eq!(
            state.last_notification_utc,
            Some(
                NaiveDateTime::parse_from_str("2020-01-01T01:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
        assert_eq!(state.reminder_number, 1);

        // Now save as json
        let json_string = state.to_json_string().unwrap();
        let result = SensorState::from_string(&json_string).unwrap();
        assert_eq!(state, result);
    }
}
