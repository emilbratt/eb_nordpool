use chrono::{DateTime, NaiveDate, NaiveDateTime, Timelike, Duration};

use chrono_tz::Tz;

use crate::region_time::tz_from_region;

const NORDPOOL_DEFAULT_TZ_REGION: &str = "Oslo";

/// This enum is used to find out how many hours in the day for price data.
/// It is always 24 hours, except when it is 23 or 25 hours..
/// For example, in spring we handle the transition from cet to cest.
/// Due to weird structure of the dataset, it is not given how many hours we have..
/// E.g., there are no clear indicators in the dataset to go after.
/// We use region and some time calculations to determine this.
pub enum Hours {
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

impl Hours {
    pub fn new(d: NaiveDate, region: &str) -> Self {
        let region = if region == "SYS" {
            NORDPOOL_DEFAULT_TZ_REGION
        } else {
            region
        };

        let dt: DateTime<Tz> = match tz_from_region(&region) {
            Ok(tz) => NaiveDateTime::from(d).and_local_timezone(tz).unwrap(),
            Err(e) => panic!("{:?}", e),
        };

        match dt.with_hour(0) {
            Some(dt) if (dt+Duration::hours(23)).hour() == 0 => Self::TwentyThree,
            Some(dt) if (dt+Duration::hours(25)).hour() == 0 => Self::TwentyFive,
            _ => Self::TwentyFour,
        }
    }
}
