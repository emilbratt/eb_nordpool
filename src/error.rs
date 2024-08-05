pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    HourlyInvalidPageID,
    HourlyPriceDateMismatch,
    HourlyPriceHourMismatch,
    HourlyPriceHourMismatchCESTToCET,
    HourlyPriceRegionNotFound,
    HourlyPriceFilteredRowsExceededTwo,

    RegionIndexNotFound,
    RegionTzNotSupported,
}
