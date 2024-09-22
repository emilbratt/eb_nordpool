use std::fs;

use eb_nordpool::{elspot, error::HourlyError, units};

#[test]
fn eur_24h() {
    // testing standard 24h days data.
    let data = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();
    assert_eq!("EUR", data.currency());
    assert_eq!("2024-06-20", data.date().to_string());
    assert!(!data.is_preliminary());
    assert!(data.has_region("Tr.heim"));
    assert!(data.has_region("SE1"));
    assert!(data.has_region("FI"));
    assert!(data.has_region("BE"));

    let prices = data.extract_prices_for_region("Tr.heim");
    let p = &prices[1];
    assert_eq!("5,82", p.value);
    assert_eq!(data.date(), p.date);

    let prices = data.extract_prices_for_region("FI");
    let p = &prices[2];
    assert_eq!("-5,00", p.value);
    assert_eq!(data.date(), p.date);

    let prices_all = data.extract_all_prices();
    for prices in prices_all {
        assert_eq!(prices.len(), 24);
        for p in prices {
            let (from, _) = p.from_to();
            assert_eq!(data.date(), from.date_naive());
        }
    }
}

#[test]
fn nok_25h() {
    // Test when we have 25 hours in a day.
    let data = elspot::hourly::from_file("./tests/data/NOK_25H.json").unwrap();
    assert!(data.is_preliminary());

    let price = data.extract_prices_for_region("Tr.heim");
    let p = &price[3];
    assert_eq!("167,66", p.value);

    for region in data.regions() {
        match region {
            // Test data has no prices for these regions..
            "AT" | "BE" | "DE-LU" | "FR" |"NL" => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 0);
            },
            _ => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 25);
                if region != "SYS" {
                    for p in prices {
                        let (from, _) = p.from_to();
                        assert_eq!(data.date(), from.date_naive());
                    }
                }
             }
        }
    }
}

#[test]
fn nok_23h() {
    // Test when we have 23 hours in a day.
    let data = elspot::hourly::from_file("./tests/data/NOK_23H.json").unwrap();
    assert!(data.is_preliminary());
    assert_eq!("2023-03-26", data.date().to_string());

    let prices = data.extract_prices_for_region("Oslo");
    assert_eq!(prices.len(), 23);
    let p = &prices[0];
    let (from, to) = p.from_to();
    assert_eq!(from.to_rfc3339(), "2023-03-26T00:00:00+01:00");
    assert_eq!(to.to_rfc3339(), "2023-03-26T01:00:00+01:00");

    let data = elspot::hourly::from_file("./tests/data/NOK_23H.json").unwrap();
    for region in data.regions() {
        match region {
            // Test data has no prices for these regions..
            "AT" | "BE" | "DE-LU" | "FR" |"NL" => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 0);
            },
            _ => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 23);
                for p in prices {
                    let (from, _) = p.from_to();
                    assert_eq!(data.date(), from.date_naive());
                }
            }
        }
    }
}

#[test]
fn hourly_invalid_json() {
    let mut json_str = fs::read_to_string("./tests/data/NOK_23H.json").unwrap();

    // Mess up the json file by adding a trailing character..
    json_str.push('!');

    let data = elspot::hourly::from_json(&json_str);
    match data {
        Ok(_) => panic!(),
        Err(e) => assert!(matches!(e, HourlyError::InvalidJSON)),
    }
}

#[test]
fn hourly_to_json_string() {
    let data = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();

    // Save data to a string (serialized json).
    let s = data.to_json_string();

    // We just reload the string and see if it works, unwrap() will fail if Err.
    elspot::hourly::from_json(&s).unwrap();
}

#[test]
fn units() {
    let data = elspot::hourly::from_file("./tests/data/EUR_24H.json").unwrap();

    let mut prices = data.extract_prices_for_region("Oslo");
    let mut p = &mut prices[1];
    p.value = String::from("0167,680"); // Change to a value with trailing zero, we should be able to handle it.

    units::convert_to_currency_fraction(&mut p);
    assert_eq!("16768", p.value);
    assert_eq!(16768f32, p.as_f32());
    assert_eq!("Cent", p.currency_unit.to_string());
    assert_eq!("MWh", p.power_unit.to_string());

    units::convert_to_kwh(&mut p);
    assert_eq!("16,768", p.value);
    assert_eq!(17_f32, p.as_f32());
    assert_eq!("kWh", p.power_unit.to_string());

    units::convert_to_currency_full(&mut p);
    assert_eq!("0,16768", p.value);
    assert_eq!(0.17_f32, p.as_f32());
    assert_eq!("Eur.", p.currency_unit.to_string());
    assert_eq!("kWh", p.power_unit.to_string());

    units::convert_to_mwh(&mut p);
    assert_eq!(167.68_f32, p.as_f32());
    assert_eq!("167,68", p.value);
    assert_eq!("MWh", p.power_unit.to_string());

    p.value = String::from("10,505");
    units::convert_to_currency_fraction(&mut p);
    assert_eq!(1051_f32, p.as_f32());
    assert_eq!("Cent", p.currency_unit.to_string());

    p.value = String::from("10,5");
    assert_eq!(11_i32, p.as_i32());
    p.value = String::from("-10,5");
    assert_eq!(-11_i32, p.as_i32());
    p.value = String::from("-10,49");
    assert_eq!(-10_i32, p.as_i32());
}
