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

#[test]
fn units() {
    let mut p = elspot::Price {
        region: String::from("Oslo"), // Region not important here..
        from: common::dummy_naive_datetime(), // Time not important here..
        to: common::dummy_naive_datetime(), // Time not important here..
        value: String::from("0167,660"),
        unit: String::from("NOK/MWh"),
    };
    assert!("0167,660" == p.value && "NOK/MWh" == p.unit);

    units::to_currency_sub_unit(&mut p);
    assert!("16766" == p.value && "NOK(1%)/MWh" == p.unit);

    units::to_power_kwh_unit(&mut p);
    assert!("16,766" == p.value && "NOK(1%)/kWh" == p.unit);

    units::to_currency_full_unit(&mut p);
    assert!("0,16766" == p.value && "NOK/kWh" == p.unit);

    units::to_power_mwh_unit(&mut p);
    assert!("167,66" == p.value && "NOK/MWh" == p.unit);

    (p.value, p.unit) = (String::from("0,500"), String::from("NOK/MWh"));
    units::to_currency_sub_unit(&mut p);
    assert!("50" == p.value && "NOK(1%)/MWh" == p.unit);

    (p.value, p.unit) = (String::from("50,0"), String::from("NOK(1%)/kWh"));
    units::to_currency_full_unit(&mut p);
    assert!("0,5" == p.value && "NOK/kWh" == p.unit);

    (p.value, p.unit) = (String::from("50,0"), String::from("NOK(1%)/kWh"));
    units::to_power_mwh_unit(&mut p);
    assert!("50000" == p.value && "NOK(1%)/MWh" == p.unit);

    (p.value, p.unit) = (String::from("0,00"), String::from("NOK/kWh"));
    units::to_currency_sub_unit(&mut p);
    assert!("00" == p.value && "NOK(1%)/kWh" == p.unit);

    (p.value, p.unit) = (String::from("0,00"), String::from("NOK(1%)/kWh"));
    units::to_currency_full_unit(&mut p);
    assert!("0" == p.value && "NOK/kWh" == p.unit);

    (p.value, p.unit) = (String::from("000"), String::from("NOK(1%)/kWh"));
    units::to_currency_full_unit(&mut p);
    assert!("0,00" == p.value && "NOK/kWh" == p.unit);

    (p.value, p.unit) = (String::from("10"), String::from("NOK/kWh"));
    units::to_currency_sub_unit(&mut p);
    assert!("1000" == p.value && "NOK(1%)/kWh" == p.unit);
}
