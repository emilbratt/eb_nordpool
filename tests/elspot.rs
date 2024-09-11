use std::fs;
use eb_nordpool::{elspot, error, region_time, units};
mod common;

#[test]
fn eur_24h() {
    // Most of the testing happens here with the standard 24h. data.
    let data = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();

    assert!(data.has_region("Tr.heim"));
    assert!(!data.prices_are_today_for_region("Tr.heim"));

    let utc_dt = common::utc_dt_for_eur_24_hour();

    let mut price = data.price_for_region_at_utc_dt("Tr.heim", &utc_dt).unwrap();
    assert_eq!("5,82", price.value);
    assert_eq!("EUR", price.currency_unit.country_str());

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

    let prices = data.all_prices_for_region("Oslo");
    assert!(prices.len() == 23);
}

#[test]
fn hourly_invalid_json() {
    let mut json_str = fs::read_to_string("./tests/data/NOK_23H.json").unwrap();

    // Mess up the json file by adding a trailing character..
    json_str.push('!');

    let data = elspot::hourly::from_json(&json_str);
    match data {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(e, error::HourlyError::InvalidJSON),
    }
}

#[test]
fn hourly_to_string() {
    let data = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();

    // Save data to a string (serialized json).
    let s = data.to_json_string();

    // We just reload the string and see if it works, unwrap() will fail if Err.
    elspot::hourly::from_json(&s).unwrap();
}

#[test]
fn region_time() {
    // Just run the function. It ensures it will not be altered without a test failure.
    region_time::utc_with_ymd_and_hms(2024, 6, 20, 11, 0, 0);
}

#[test]
fn units() {
    let mut p = elspot::Price {
        region: String::from("Oslo"), // Region not important here..
        from: common::dummy_naive_datetime(), // Time not important here..
        to: common::dummy_naive_datetime(), // Time not important here..
        value: String::from("0167,680"), // Add trailing zero, we should be able to handle it.
        currency_unit: units::Currency::NOK(units::CurrencyUnit::Full),
        power_unit: units::Power::MWh,
    };

    units::convert_to_currency_fraction(&mut p);
    assert_eq!("16768", p.value);
    assert_eq!(16768f32, p.as_f32());
    assert_eq!("Øre", p.currency_unit.to_str());
    assert_eq!("MWh", p.power_unit.to_str());

    units::convert_to_kwh(&mut p);
    assert_eq!("16,768", p.value);
    assert_eq!(17f32, p.as_f32());
    assert_eq!("kWh", p.power_unit.to_str());

    units::convert_to_currency_full(&mut p);
    assert_eq!("0,16768", p.value);
    assert_eq!(0.17f32, p.as_f32());
    assert_eq!("Kr.", p.currency_unit.to_str());
    assert_eq!("kWh", p.power_unit.to_str());

    units::convert_to_mwh(&mut p);
    assert_eq!(167.68f32, p.as_f32());
    assert_eq!("167,68", p.value);
    assert_eq!("MWh", p.power_unit.to_str());

    p.value = String::from("10,505");
    units::convert_to_currency_fraction(&mut p);
    assert_eq!(1051f32, p.as_f32());
    assert_eq!("Øre", p.currency_unit.to_str());
}
