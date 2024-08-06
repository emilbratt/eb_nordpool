use std::fmt;
use core::result::Result;

pub type HourlyResult<T> = Result<T, HourlyError>;

#[derive(Clone, Debug, PartialEq)]
pub enum HourlyError {
    InvalidJSON,
    InvalidPageID,
    PriceDateMismatch,
    PriceHourMismatch,
    PriceHourMismatchCESTToCET,
    PriceRegionNotFound,
    PriceFilteredRowsExceededTwo,
}

impl fmt::Display for HourlyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type RegionResult<T> = Result<T, RegionError>;

#[derive(Clone, Debug, PartialEq)]
pub enum RegionError {
    RegionIndexNotFound,
    RegionTzNotSupported,
}

impl fmt::Display for RegionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
