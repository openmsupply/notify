use serde::{Deserialize, Serialize};

use crate::ColdChainError;

/* Example Config
{
    "confirmOk": true,
    "highTemp": true,
    "id": "262cd899-7abe-4364-ae83-c3eeae2b63c4",
    "kind": "COLD_CHAIN",
    "lowTemp": true,
    "noData": true,
    "noDataInterval": 1,
    "noDataUnits": "hours",
    "parameters": "{}",
    "parsedParameters": {},
    "recipientIds": [
        "b43a2a02-0f47-4da3-a0f5-f3abc6a626eb",
        "b33fe58d-520f-454f-9048-9cc1c837c82f"
    ],
    "recipientListIds": [],
    "remind": true,
    "reminderInterval": 15,
    "reminderUnits": "minutes",
    "sensorIds": [
        "8a31c952-77cb-455c-be09-d6cabb402059",
        "c6ef6e47-245a-49a6-9c34-ddbd0d0a3d7c",
        "9329b3d1-b4aa-4cf7-88a6-f43e72d2f312",
        "16b07349-6dfb-4d45-9eef-d670e0129628",
        "93d3053b-6549-4a35-8bbe-2c37b143f7f3",
        "364e8168-94e3-498a-92b0-67ef014f6398",
        "392dc22b-14bb-4681-b77f-f0c7394f4b03",
        "3737ac8a-c6a2-41f3-8286-055322e31eee",
        "76791e2d-351c-47be-b390-e7559eb5fdd2"
    ],
    "sqlRecipientListIds": [],
    "status": "ENABLED",
    "title": "Cold Chain Alerts - Central Hospital"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColdChainPluginConfig {
    #[serde(default)]
    pub high_temp: bool,
    #[serde(default)]
    pub low_temp: bool,
    #[serde(default)]
    pub sensor_ids: Vec<String>,
    #[serde(default = "default_low_temp_limit")]
    pub low_temp_threshold: f64,
    #[serde(default = "default_high_temp_limit")]
    pub high_temp_threshold: f64,
    #[serde(default)]
    pub no_data: bool,
    #[serde(default = "default_no_data_interval")]
    pub no_data_interval: u32,
    #[serde(default = "default_no_data_units")]
    pub no_data_units: String,
}

fn default_low_temp_limit() -> f64 {
    2.0
}

fn default_high_temp_limit() -> f64 {
    8.0
}

fn default_no_data_interval() -> u32 {
    1
}

fn default_no_data_units() -> String {
    "hours".to_string()
}

impl ColdChainPluginConfig {
    pub fn from_string(json_string: &str) -> Result<Self, ColdChainError> {
        let config: ColdChainPluginConfig = serde_json::from_str(json_string)
            .map_err(|e| ColdChainError::UnableToParseConfig(format!("{:?}", e)))?;

        Ok(config)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_broken_parse_config() {
        let result = ColdChainPluginConfig::from_string("{sdf}");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_config_high_temp() {
        let example1 = r#"{
    "confirmOk": true,
    "highTemp": true,
    "id": "262cd899-7abe-4364-ae83-c3eeae2b63c4",
    "kind": "COLD_CHAIN",
    "lowTemp": true,
    "noData": true,
    "noDataInterval": 1,
    "noDataUnits": "hours",
    "parameters": "{}",
    "parsedParameters": {},
    "recipientIds": [
        "b43a2a02-0f47-4da3-a0f5-f3abc6a626eb",
        "b33fe58d-520f-454f-9048-9cc1c837c82f"
    ],
    "recipientListIds": [],
    "remind": true,
    "reminderInterval": 15,
    "reminderUnits": "minutes",
    "sensorIds": [
        "8a31c952-77cb-455c-be09-d6cabb402059",
        "c6ef6e47-245a-49a6-9c34-ddbd0d0a3d7c",
        "9329b3d1-b4aa-4cf7-88a6-f43e72d2f312",
        "16b07349-6dfb-4d45-9eef-d670e0129628",
        "93d3053b-6549-4a35-8bbe-2c37b143f7f3",
        "364e8168-94e3-498a-92b0-67ef014f6398",
        "392dc22b-14bb-4681-b77f-f0c7394f4b03",
        "3737ac8a-c6a2-41f3-8286-055322e31eee",
        "76791e2d-351c-47be-b390-e7559eb5fdd2"
    ],
    "sqlRecipientListIds": [],
    "status": "ENABLED",
    "title": "Cold Chain Alerts - Central Hospital"
}"#;

        let result = ColdChainPluginConfig::from_string(example1);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.high_temp);
        assert!(config.low_temp);
        assert_eq!(config.sensor_ids.len(), 9);
    }
}
