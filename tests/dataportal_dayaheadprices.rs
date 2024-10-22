#![allow(unused)]

use std::fs;

use eb_nordpool::{
    elspot::dataportal_dayaheadprices,
    error::ElspotError,
    units,
};

#[test]
fn dummy() {
    let data = dataportal_dayaheadprices::from_json("");

    assert!(matches!(data, Err(ElspotError::DataPortalDayaheadPricesNotImplemented)));
}
