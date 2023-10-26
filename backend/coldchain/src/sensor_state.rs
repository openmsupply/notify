/* This file contains a Struct to store sensor state, Ok, HighTemp, LowTemp, or NoData along with a timestamp */

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::ColdChainError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SensorState {
    pub sensor_id: String,
    pub status: SensorStatus,
    pub timestamp: NaiveDateTime,
    pub temperature: Option<f64>,
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
    fn test_parse_status_ok() {
        let example1 =
            r#"{ "sensor_id": "1234", "status": "Ok", "timestamp": "2020-01-01T00:00:00" }"#;
        let result = SensorState::from_string(example1);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.sensor_id, "1234");
        assert_eq!(state.status, SensorStatus::Ok);
        assert_eq!(
            state.timestamp,
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
            state.timestamp,
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
            state.timestamp,
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
            state.timestamp,
            NaiveDateTime::parse_from_str("2020-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        );

        // Now save as json
        let json_string = state.to_json_string().unwrap();
        let result = SensorState::from_string(&json_string).unwrap();
        assert_eq!(state, result);
    }
}
