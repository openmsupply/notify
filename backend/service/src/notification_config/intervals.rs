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
