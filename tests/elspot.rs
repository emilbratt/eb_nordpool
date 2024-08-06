use std::fs;
use eb_nordpool::{elspot, error};
mod common;

#[test]
fn eur_24h() {
    let hourly = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();

    assert!(hourly.has_region("Tr.heim"));
    assert!(!hourly.prices_are_for_today("Tr.heim"));

    let utc_dt = common::utc_dt_for_eur_24_hour();

    let mut price: elspot::Price;

    price = hourly.price_for_region_at_utc_dt("Tr.heim", &utc_dt).unwrap();
    assert_eq!("5,82", price.value);
    assert_eq!("EUR/MWh", price.unit);

    price = hourly.price_for_region_at_utc_dt("FI", &utc_dt).unwrap();
    assert_eq!("-5,00", price.value);

    let prices = hourly.all_prices_for_region("FR");

    assert!(prices.len() == 24);
}

#[test]
fn nok_25h() {
    let hourly = elspot::hourly::from_file("./tests/data/NOK_25H.json").unwrap();

    let utc_dt = common::utc_dt_for_nok_25_hour();

    let price = hourly.price_for_region_at_utc_dt("Tr.heim", &utc_dt).unwrap();
    assert_eq!("167,66", price.value);
    assert_eq!("NOK/MWh", price.unit);

    let prices: Vec<elspot::Price> = hourly.all_prices_for_region("Oslo");
    assert!(prices.len() == 25);
}

#[test]
fn nok_23h() {
    let hourly = elspot::hourly::from_file("./tests/data/NOK_23H.json").unwrap();

    let utc_dt = common::utc_dt_for_nok_23_hour();

    let price = hourly.price_for_region_at_utc_dt("Oslo", &utc_dt).unwrap();
    assert_eq!("868,18", price.value);
    assert_eq!("NOK/MWh", price.unit);

    let prices = hourly.all_prices_for_region("Oslo");
    assert!(prices.len() == 23);
}

#[test]
fn hourly_invalid_json() {
    let json_str = fs::read_to_string("./tests/data/NOK_23H.json").unwrap();

    let invalid = String::from(json_str + "!"); // Mess up the json file by adding a trailing character..

    let hourly = elspot::hourly::from_json(&invalid);

    match hourly {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(e, error::HourlyError::InvalidJSON),
    }
}
