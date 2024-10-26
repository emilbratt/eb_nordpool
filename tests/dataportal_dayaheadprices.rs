#![allow(unused)]

use std::fs;

use eb_nordpool::{
    elspot::dataportal_dayaheadprices,
    error::ElspotError,
    units,
};

#[test]
fn from_file() {
    let data = dataportal_dayaheadprices::from_file("./tests/data/dataportal_dayaheadprices_NOK.json");
    match data {
        Ok(data) => {
            assert!(!data.is_preliminary());
            assert!(data.has_region("DK1"));
            assert!(data.has_region("NO3"));
            assert_eq!(data.currency(), "NOK");

            let mut prices = data.extract_prices_for_region("NO3");
            assert_eq!("182.94", prices[5].value);

            for mut p in prices.iter_mut() {
                let v = p.as_f32();
                assert!(v > 170.0);
                assert!(v < 230.0);

                units::convert_to_kwh(&mut p);
                let v = p.as_f32();
                assert!(v > 0.17);
                assert!(v < 0.23);

                units::convert_to_currency_fraction(&mut p);
                let v = p.as_f32();
                assert!(v > 17.0);
                assert!(v < 23.0);

                units::convert_to_mwh(&mut p);
                let v = p.as_f32();
                assert!(v < 23000.0);
                assert!(v > 17000.0);
            }
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

#[test]
fn to_json_string() {
    let data = dataportal_dayaheadprices::from_file("./tests/data/dataportal_dayaheadprices_NOK.json").unwrap();

    // Save data to a string (serialized json).
    let s = data.to_json_string();

    // We just reload the string and see if it works, unwrap() will fail if Err.
    dataportal_dayaheadprices::from_json(&s).unwrap();
}

// #[test]
// fn from_nordpool() {
//     let date = "2024-10-24";
//     let currency = "NOK";
//     let mut regions: Vec<&str> = vec![];
//     for region in dataportal_dayaheadprices::regions::SUPPORTED_REGIONS.iter() {
//         regions.push(region);
//     }
//     let data = dataportal_dayaheadprices::from_nordpool(currency, date, regions).unwrap();

//     let mut regions = data.extract_prices_all_regions();
//     for prices in regions.iter() {
//         for p in prices.iter() {
//             println!("{}: {}", p.region, p.price_label());
//         }
//     }
// }
