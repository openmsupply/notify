use std::fmt::Display;

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

impl Display for IntervalUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntervalUnits::Minutes => write!(f, "minutes"),
            IntervalUnits::Hours => write!(f, "hours"),
            IntervalUnits::Days => write!(f, "days"),
            IntervalUnits::Weeks => write!(f, "weeks"),
            IntervalUnits::Months => write!(f, "months"),
            IntervalUnits::Years => write!(f, "years"),
        }
    }
}

impl IntervalUnits {
    pub fn to_duration(&self, interval: u32) -> chrono::Duration {
        match self {
            IntervalUnits::Minutes => chrono::Duration::minutes(interval as i64),
            IntervalUnits::Hours => chrono::Duration::hours(interval as i64),
            IntervalUnits::Days => chrono::Duration::days(interval as i64),
            IntervalUnits::Weeks => chrono::Duration::weeks(interval as i64),
            IntervalUnits::Months => chrono::Duration::days(interval as i64 * 30),
            IntervalUnits::Years => chrono::Duration::days(interval as i64 * 365),
        }
    }
}
