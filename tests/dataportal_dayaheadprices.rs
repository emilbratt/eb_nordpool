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
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}
