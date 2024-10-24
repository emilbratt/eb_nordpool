use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::{RegionResult, RegionError};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Region {
    // Baltic
    EE,
    LT,
    LV,
    CWE,
    AT,
    BE,
    FR,
    GER,
    NL,
    PL,

    // Nordic
    DK1,
    DK2,
    FI,
    NO1,
    NO2,
    NO3,
    NO4,
    NO5,
    SE1,
    SE2,
    SE3,
    SE4,

    // System
    SYS,
}

impl Region {
    pub fn from_str(region: &str) -> RegionResult<Self> {
        match region {
            // Baltic
            "EE" => Ok(Self::EE),
            "LT" => Ok(Self::LT),
            "LV" => Ok(Self::LV),
            "CWE" => Ok(Self::CWE),
            "AT" => Ok(Self::AT),
            "BE" => Ok(Self::BE),
            "FR" => Ok(Self::FR),
            "GER" => Ok(Self::GER),
            "NL" => Ok(Self::NL),
            "PL" => Ok(Self::PL),

            // Nordic
            "DK1" => Ok(Self::DK1),
            "DK2" => Ok(Self::DK2),
            "FI" => Ok(Self::FI),
            "NO1" => Ok(Self::NO1),
            "NO2" => Ok(Self::NO2),
            "NO3" => Ok(Self::NO3),
            "NO4" => Ok(Self::NO4),
            "NO5" => Ok(Self::NO5),
            "SE1" => Ok(Self::SE1),
            "SE2" => Ok(Self::SE2),
            "SE3" => Ok(Self::SE3),
            "SE4" => Ok(Self::SE4),

            // System
            "SYS" => Ok(Self::SYS),

            _ => Err(RegionError::RegionNotSupported),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const REGION_LIST: [&str; 23] = [
    // Baltic
    "EE",
    "LT",
    "LV",
    "CWE",
    "AT",
    "BE",
    "FR",
    "GER",
    "NL",
    "PL",

    // Nordic
    "DK1",
    "DK2",
    "FI",
    "NO1",
    "NO2",
    "NO3",
    "NO4",
    "NO5",
    "SE1",
    "SE2",
    "SE3",
    "SE4",

    // System
    "SYS",
];

pub fn print_all() {
    println!("All Supported regions:");
    for region in REGION_LIST {
        println!("'{}' ", region);
    }
    println!();
}
