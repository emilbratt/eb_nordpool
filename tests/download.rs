
use chrono::Local;

use eb_nordpool::{
    elspot::{self, dataportal_dayaheadprices},
    units,
};

// #[test]
#[allow(unused)] // Uncomment the test attribute (line above) to include this test.
fn from_nordpool() {
    let date = Local::now().format("2024-12-20").to_string();
    let currency = "RON";
    let mut regions: Vec<&str> = vec![];
    for region in dataportal_dayaheadprices::regions::SUPPORTED_REGIONS.iter() {
        regions.push(region);
    }
    let regions: Vec<&str> = vec!["TEL"]; // UN-COMMENT TO OVERRIDE AND USE REGION.

    let data = elspot::from_nordpool(currency, &date, &regions).unwrap();
    let mut regions = data.extract_prices_all_regions();

    println!("Date: {}\n", data.date());
    for prices in regions.iter_mut() {
        for mut p in prices.iter_mut() {
            println!("{}: {} | float: {}", p.region, p.price_label(), p.as_f32());
            units::convert_to_kwh(&mut p);
            units::convert_to_currency_fraction(&mut p);
            println!("{}: {} | float: {}\n", p.region, p.price_label(), p.as_f32());
        }
    }

    println!("date: {}", data.date());
    println!("currency: {}", data.currency());
    println!("is final: {}", data.is_final());
}
