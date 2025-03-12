use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Region {
    // Baltic
    EE,
    LT,
    LV,

    // Central Western Europe
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

    // Romania
    TEL,

    // System
    SYS,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const SUPPORTED_REGIONS: [&str; 22] = [
    // Baltic
    "EE",
    "LT",
    "LV",

    // Central Western Europe
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

    // Romania
    // "TEL",

    // System
    "SYS",
];

pub fn list_supported() {
    println!("Supported regions");
    for r in SUPPORTED_REGIONS {
        println!("'{}' ", r);
    }
    println!();
}
