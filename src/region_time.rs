use crate::error::{
    RegionError,
    RegionResult
};

use chrono::{
    Utc,
    offset::TimeZone,
    Local,
    DateTime,
};

use chrono_tz::{
    Tz,
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

fn tz_from_region(region: &str) -> RegionResult<Tz> {
    match region {
        "Oslo" | "Bergen" | "Kr.sand" | "Molde" | "Tr.heim" | "Tromsø" => Ok(Oslo),
        "NO1" | "NO2" | "NO3" | "NO4" | "NO5" => Ok(Oslo),
        "SE1" | "SE2" | "SE3" | "SE4" => Ok(Stockholm),
        "DK1" | "DK2" => Ok(Copenhagen),
        "FI" => Ok(Helsinki),
        "EE" => Ok(Tallinn),
        "LV" => Ok(Riga),
        "LT" => Ok(Vilnius),
        "AT" => Ok(Vienna),
        "BE" => Ok(Brussels),
        "DE-LU" => Ok(Luxembourg),
        "FR" => Ok(Paris),
        "NL" => Ok(Amsterdam),
        _ => Err(RegionError::RegionTzNotSupported),
    }
}

pub fn region_dt_now_from_region(region: &str) -> DateTime<Tz> {
    match tz_from_region(region) {
        Ok(tz) => Local::now().with_timezone(&tz),
        Err(e) => panic!("{:?}", e),
    }
}

pub fn region_dt_from_utc_dt(region: &str, utc_dt: &DateTime<Utc>) -> DateTime<Tz> {
    match tz_from_region(region) {
        Ok(tz) => utc_dt.with_timezone(&tz),
        Err(e) => panic!("{:?}", e),
    }
}

pub fn utc_with_ymd_and_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, min, sec).unwrap()
}
