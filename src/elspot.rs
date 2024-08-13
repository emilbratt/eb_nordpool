use std::fmt;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub mod hourly;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Currencies {
    DKK,
    EUR,
    NOK,
    SEK,
}

impl fmt::Display for Currencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Prices returned comes in the form of this datatype.
#[derive(Debug)]
pub struct Price {
    pub region: String,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub value: String,
    pub unit: String,
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
