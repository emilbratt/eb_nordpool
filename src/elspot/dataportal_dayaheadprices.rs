#![allow(unused)]

use std::fs;

use crate::error::{
    ElspotError,
    ElspotResult,
    RegionError,
    RegionResult,
};


use serde::{Deserialize, Serialize};
use serde_json;

use reqwest;

const NORDPOOL_BASE_URL: &str = "https://dataportal-api.nordpoolgroup.com/api/DayAheadPrices";

pub fn from_json(json_str: &str) -> ElspotResult<MarkedData> {
    MarkedData::new(json_str)
}

pub fn from_file(path: &str) -> ElspotResult<MarkedData> {
    let json_str = fs::read_to_string(path).unwrap();

    from_json(&json_str)
}

pub fn from_url(url: &str) -> ElspotResult<MarkedData> {
    let r = reqwest::blocking::get(url).unwrap();

    let json_str = r.text().unwrap();

    from_json(&json_str)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkedData {}

impl MarkedData {
    pub fn new(json_str: &str) -> ElspotResult<Self> {
        match serde_json::from_str::<Self>(json_str) {
            Ok(data) => Err(ElspotError::DataPortalDayaheadPricesNotImplemented),
            Err(_) => Err(ElspotError::DataPortalDayaheadPricesNotImplemented),
        }
    }
}
