use chrono::{Duration, TimeZone, Utc};

use eb_nordpool::{elspot::Price, units};

fn get_dummy_price(v: &str) -> Price {
    let dt = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

    Price {
        value: String::from(v),
        from: dt,
        to: dt + Duration::hours(1),
        date: dt.date_naive(),
        region: String::from("NO3"),
        currency_unit: units::Currency::NOK(units::CurrencyUnit::Full),
        market_time_unit: units::Mtu::Sixty,
        power_unit: units::Power::MWh,
    }
}

#[test]
fn converting_price() {
    let mut p = get_dummy_price("0167.680");

    units::convert_to_currency_fraction(&mut p);
    assert_eq!("16768", p.value);
    assert_eq!(16768f32, p.as_f32());
    assert_eq!(16768f64, p.as_f64());
    assert_eq!("Øre", p.currency_unit.as_str());
    assert_eq!("MWh", p.power_unit.as_str());

    units::convert_to_kwh(&mut p);
    assert_eq!("16.768", p.value);
    assert_eq!(17_f32, p.as_f32());
    assert_eq!(17_f64, p.as_f64());
    assert_eq!("kWh", p.power_unit.as_str());

    units::convert_to_currency_full(&mut p);
    assert_eq!("0.16768", p.value);
    assert_eq!(0.17_f32, p.as_f32());
    assert_eq!(0.17_f64, p.as_f64());
    assert_eq!("Kr.", p.currency_unit.as_str());
    assert_eq!("kWh", p.power_unit.as_str());

    units::convert_to_mwh(&mut p);
    assert_eq!(167.68_f32, p.as_f32());
    assert_eq!(167.68_f64, p.as_f64());
    assert_eq!("167.68", p.value);
    assert_eq!("MWh", p.power_unit.as_str());

    p.value = String::from("10.505");
    units::convert_to_currency_fraction(&mut p);
    assert_eq!(1051_f32, p.as_f32());
    assert_eq!(1051_f64, p.as_f64());
    assert_eq!("Øre", p.currency_unit.as_str());

    p.value = String::from("10.5");
    assert_eq!(11_i32, p.as_i32());
    assert_eq!(11_f64, p.as_f64());
    assert_eq!(11_i64, p.as_i64());
}

#[test]
fn negative_price() {
    let mut p = get_dummy_price("-0.6");
    units::convert_to_currency_fraction(&mut p);
    assert_eq!(-60_f32, p.as_f32());
    assert_eq!(-60_i32, p.as_i32());

    units::convert_to_kwh(&mut p);
    assert_eq!(-0.0_f32, p.as_f32());
    assert_eq!(-0_i32, p.as_i32());

    let mut p = get_dummy_price("-30");
    units::convert_to_kwh(&mut p);
    assert_eq!(-0.03_f32, p.as_f32());
    assert_eq!(-0_i32, p.as_i32());

    units::convert_to_currency_fraction(&mut p);
    assert_eq!(-3_f32, p.as_f32());
    assert_eq!(-3_i32, p.as_i32());


    let p = get_dummy_price("-10.5");
    assert_eq!(-10.5_f32, p.as_f32());
    assert_eq!(-11_i32, p.as_i32());

    let p = get_dummy_price("-10.49");
    assert_eq!(-10.49_f32, p.as_f32());
    assert_eq!(-10_i32, p.as_i32());
}