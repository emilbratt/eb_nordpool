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
