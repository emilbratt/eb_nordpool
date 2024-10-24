use crate::units;

const NORDPOOL_BASE_URL: &str = "https://dataportal-api.nordpoolgroup.com/api/DayAheadPrices";

pub struct QueryOptions<'a> {
    currency: &'a str,
    date: &'a str,
    regions: &'a Vec<&'a str>,
}

impl <'a>QueryOptions<'a> {
    pub fn new(currency: &'a str, date: &'a str, regions: &'a Vec<&'a str>) -> Self {
        Self {
            currency,
            date,
            regions,
        }
    }

    pub fn build(&self) -> String {
        todo!()
        // ..?date=2024-09-22&market=DayAhead&deliveryArea=DK1,NO3&currency=NOK
        // format!("{}", NORDPOOL_BASE_URL)
    }
}
