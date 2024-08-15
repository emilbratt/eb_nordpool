use std::fs;
use eb_nordpool::{elspot, error, region_time};
mod common;

#[test]
fn eur_24h() {
    // Most of the testing happens here with the standard 24h. data.
    let data = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();

    assert!(data.has_region("Tr.heim"));
    assert!(!data.prices_are_today_for_region("Tr.heim"));

    let utc_dt = common::utc_dt_for_eur_24_hour();

    let mut price: elspot::Price;

    price = data.price_for_region_at_utc_dt("Tr.heim", &utc_dt).unwrap();
    assert_eq!("5,82", price.value);
    assert_eq!("EUR/MWh", price.unit);

    price = data.price_for_region_at_utc_dt("FI", &utc_dt).unwrap();
    assert_eq!("-5,00", price.value);

    for region in data.regions() {
        let prices = data.all_prices_for_region(region);
        assert!(prices.len() == 24);

    }
}

#[test]
fn nok_25h() {
    // Edge case testing when we have 25 hours in a day.
    let data = elspot::hourly::from_file("./tests/data/NOK_25H.json").unwrap();

    let utc_dt = common::utc_dt_for_nok_25_hour();

    let price = data.price_for_region_at_utc_dt("Tr.heim", &utc_dt).unwrap();
    assert_eq!("167,66", price.value);
    assert_eq!("NOK/MWh", price.unit);

    let prices = data.all_prices_for_region("Oslo");
    assert!(prices.len() == 25);
}

#[test]
fn nok_23h() {
    // Edge case testing when we have 23 hours in a day.
    let data = elspot::hourly::from_file("./tests/data/NOK_23H.json").unwrap();

    let utc_dt = common::utc_dt_for_nok_23_hour();

    let price = data.price_for_region_at_utc_dt("Oslo", &utc_dt).unwrap();
    assert_eq!("868,18", price.value);
    assert_eq!("NOK/MWh", price.unit);

    let prices = data.all_prices_for_region("Oslo");
    assert!(prices.len() == 23);
}

#[test]
fn hourly_invalid_json() {
    let json_str = fs::read_to_string("./tests/data/NOK_23H.json").unwrap();

    let invalid = String::from(json_str + "!"); // Mess up the json file by adding a trailing character..

    let data = elspot::hourly::from_json(&invalid);

    match data {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(e, error::HourlyError::InvalidJSON),
    }
}

#[test]
fn hourly_to_string() {
    let data = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();

    // Save data to a string (serialized json).
    let s = data.to_string();

    // Reload the string and see if it still works.
    if let Err(e) = elspot::hourly::from_json(&s) {
        panic!("{}", e);
    }
}

#[test]
fn region_time() {
    // Just run the function. It ensures it will not be altered without a test failure.
    region_time::utc_with_ymd_and_hms(2024, 6, 20, 11, 0, 0);
}
