use crate::error::{
    RegionError,
    RegionResult
};

use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::{
    Tz,
    Etc::UTC, // "Etcetera" -> "UTC": some timezones cannot be attributed to any area..
    Europe::{
        Oslo,
        Stockholm,
        Copenhagen,
        Helsinki,
        Tallinn,
        Riga,
        Vilnius,
        Vienna,
        Brussels,
        Luxembourg,
        Paris,
        Amsterdam,
    }
};

// const NORDPOOL_TZ_REGION: &str = "Oslo";
pub fn tz_from_region(region: &str) -> RegionResult<Tz> {
    match region {
        // Nordic
        "Oslo" | "Bergen" | "Kr.sand" | "Molde" | "Tr.heim" | "Tromsø" => Ok(Oslo),
        "NO1" | "NO2" | "NO3" | "NO4" | "NO5" => Ok(Oslo),
        "SE1" | "SE2" | "SE3" | "SE4" => Ok(Stockholm),
        "DK1" | "DK2" => Ok(Copenhagen),
        "FI" => Ok(Helsinki),

        // Baltic
        "EE" => Ok(Tallinn),
        "LV" => Ok(Riga),
        "LT" => Ok(Vilnius),

        // Central Western Europe
        "AT" => Ok(Vienna),
        "BE" => Ok(Brussels),
        "DE-LU" => Ok(Luxembourg),
        "FR" => Ok(Paris),
        "NL" => Ok(Amsterdam),

        // System
        "SYS" => Ok(UTC),
        _ => Err(RegionError::RegionTzNotSupported),
    }
}

pub fn dt_region_from_utc_dt(utc_dt: &DateTime<Utc>, region: &str) -> DateTime<Tz> {
    match tz_from_region(region) {
        Ok(tz) => utc_dt.with_timezone(&tz),
        Err(e) => panic!("{:?}", e),
    }
}

pub fn dt_tz_from_naive_dt(naive: NaiveDateTime, region: &str) -> DateTime<Tz> {
    match tz_from_region(region) {
        Ok(tz) => naive.and_local_timezone(tz).unwrap(),
        Err(e) => panic!("{:?}", e),
    }
}
