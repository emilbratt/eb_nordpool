use crate::error::{
    ElspotError,
    ElspotResult,
};

// The unit string is found somewhere inside the data-set from nordpool.
const EXPECTED_UNIT_SRINGS: [&str; 4] = [
    "EUR/MWh",
    "DKK/MWh",
    "NOK/MWh",
    "SEK/MWh",
];

pub fn test_unit_string(unit_string: &str) -> ElspotResult<()> {
    if EXPECTED_UNIT_SRINGS.contains(&unit_string) {
        Ok(())
    } else {
        Err(ElspotError::MarketdataPage10InvalidUnitString)
    }
}
