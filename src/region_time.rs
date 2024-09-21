use crate::error::{
    RegionError,
    RegionResult
};

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc, Timelike, Duration};

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

const NORDPOOL_TZ_REGION: &str = "Oslo";

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
        "SYS" => Ok(UTC),
        _ => Err(RegionError::RegionTzNotSupported),
    }
}
pub enum PriceHours {
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

impl PriceHours {
    pub fn new(d: NaiveDate, region: &str) -> Self {
        let region = if region == "SYS" {
            NORDPOOL_TZ_REGION
        } else {
            region
        };

        let dt: DateTime<Tz> = match tz_from_region(&region) {
            Ok(tz) => NaiveDateTime::from(d).and_local_timezone(tz).unwrap(),
            Err(e) => panic!("{:?}", e),
        };

        // if dt.offset() == (dt+Duration::days(1)).offset() {
        //     println!("{}", dt.offset());
        //     return PriceHours::TwentyFour;
        // }

        match dt.with_hour(0) {
            Some(dt) if (dt+Duration::hours(23)).hour() == 0 => PriceHours::TwentyThree,
            Some(dt) if (dt+Duration::hours(25)).hour() == 0 => PriceHours::TwentyFive,
            _ => PriceHours::TwentyFour,
        }
    }

    pub fn as_int(&self) -> usize {
        match self {
            Self::TwentyThree => 23,
            Self::TwentyFour => 24,
            Self::TwentyFive => 25,
        }
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
