
use chrono::Local;

use eb_nordpool::{
    elspot::{self, dataportal_dayaheadprices},
    units,
};

#[test] #[ignore]
fn from_nordpool() {
    let date = Local::now().format("%Y-%m-%d").to_string();
    println!("TEST DONWLOAD");

    // Test for all regions with "EUR" as currency
    let currency = "EUR";
    let mut regions: Vec<&str> = Vec::with_capacity(dataportal_dayaheadprices::regions::SUPPORTED_REGIONS.len());
    for region in dataportal_dayaheadprices::regions::SUPPORTED_REGIONS.iter() {
        regions.push(region);
    }

    // // Test for Romania
    // let currency = "RON";
    // let regions: Vec<&str> = vec!["TEL"];

    // Test for Bulgaria
    // let currency = "BGN";
    // let regions: Vec<&str> = vec!["BG"];

    let data = elspot::from_nordpool(currency, &date, &regions).unwrap();
    let mut regions = data.extract_prices_all_regions();

    println!("Date: {}\n", data.date());
    for prices in regions.iter_mut() {
        for mut p in prices.iter_mut() {
            let (from, _) = p.from_to();
            println!("{from}");
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
