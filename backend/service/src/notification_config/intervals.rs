use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntervalUnits {
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}
