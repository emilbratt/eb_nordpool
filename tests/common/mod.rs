use chrono::{
    offset::TimeZone,
    Utc,
    DateTime,
    NaiveDateTime,
    Duration,
};

use chrono_tz::{Tz, Europe};

pub fn utc_dt_for_eur_24_hour() -> DateTime<Utc> {
    // Date for eur 24 hour: 2024-06-20
    let dt = "2024-06-19 23:00:00";
    let fmt = "%Y-%m-%d %H:%M:%S";
    let naive_dt = NaiveDateTime::parse_from_str(dt, fmt).unwrap();

    Utc.from_utc_datetime(&naive_dt)
}

pub fn utc_dt_for_nok_23_hour() -> DateTime<Utc> {
    // Date for nok 23 hour: 2023-03-26
    let dt = "2023-03-26 00:00:00";
    let fmt = "%Y-%m-%d %H:%M:%S";
    let naive_dt = NaiveDateTime::parse_from_str(dt, fmt).unwrap();

    Utc.from_utc_datetime(&naive_dt)
}

pub fn utc_dt_for_nok_25_hour() -> DateTime<Utc> {
    // Date for nok 25 hour: 2022-10-30

    // lots of code here, but this is just for highlighting the ambiguity of the time whem going from CEST to CET.
    let mut oslo_dt:DateTime<Tz> = Europe::Oslo.with_ymd_and_hms(2022, 10, 30, 1, 0, 0).unwrap();
    oslo_dt += Duration::hours(2); // Moves time from CEST to CET (summer to winter..).

    let dt = "2022-10-30 01:00:00";
    let fmt = "%Y-%m-%d %H:%M:%S";
    let naive_dt = NaiveDateTime::parse_from_str(dt, fmt).unwrap();

    let utc_from_oslo_dt = Utc.from_utc_datetime(&oslo_dt.naive_utc());
    let utc_dt = Utc.from_utc_datetime(&naive_dt);

    assert_eq!(oslo_dt.to_utc(), utc_dt);

    assert_eq!(utc_from_oslo_dt.to_rfc3339(), utc_dt.to_rfc3339());

    // All good, return one of the values..
    utc_dt
}
