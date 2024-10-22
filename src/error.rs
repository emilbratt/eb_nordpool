use std::fmt;
use core::result::Result;

pub type ElspotResult<T> = Result<T, ElspotError>;

#[derive(Debug)]
pub enum ElspotError {
    DataPortalDayaheadPricesNotImplemented,

    MarketdataPage10InvalidJson,
    MarketdataPage10InvalidPageId,
}

impl fmt::Display for ElspotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type RegionResult<T> = Result<T, RegionError>;

#[derive(Debug)]
pub enum RegionError {
    RegionIndexNotFound,
    RegionTzNotSupported,
}

impl fmt::Display for RegionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type UnitResult<T> = Result<T, UnitError>;

#[derive(Debug)]
pub enum UnitError {
    InvalidUnitstring,
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
