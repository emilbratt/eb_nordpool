use std::{fs, fmt};

use crate::elspot::Price;
use crate::error::{
    HourlyError,
    HourlyResult,
    RegionError,
    RegionResult,
};
use crate::region_time::{dt_tz_from_naive_dt, PriceHours};
use crate::units;

use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    Duration,
};
use chrono_tz::Tz;

use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};

use reqwest;

const NORDPOOL_URL_EUR: &str = "https://www.nordpoolgroup.com/api/marketdata/page/10?currency=EUR";
const NORDPOOL_URL_DKK: &str = "https://www.nordpoolgroup.com/api/marketdata/page/10?currency=DKK";
const NORDPOOL_URL_NOK: &str = "https://www.nordpoolgroup.com/api/marketdata/page/10?currency=NOK";
const NORDPOOL_URL_SEK: &str = "https://www.nordpoolgroup.com/api/marketdata/page/10?currency=SEK";

pub fn from_json(json_str: &str) -> HourlyResult<Hourly> {
    let hourly = Hourly::new(json_str)?;

    Ok(hourly)
}

pub fn from_file(path: &str) -> HourlyResult<Hourly> {
    let json_str = fs::read_to_string(path).unwrap();

    from_json(&json_str)
}

pub fn from_url(url: &str) -> HourlyResult<Hourly> {
    let r = reqwest::blocking::get(url).unwrap();

    let json_str = r.text().unwrap();

    from_json(&json_str)
}

pub fn from_nordpool_eur() -> HourlyResult<Hourly> {
    from_url(NORDPOOL_URL_EUR)
}

pub fn from_nordpool_dkk() -> HourlyResult<Hourly> {
    from_url(NORDPOOL_URL_DKK)
}

pub fn from_nordpool_nok() -> HourlyResult<Hourly> {
    from_url(NORDPOOL_URL_NOK)
}

pub fn from_nordpool_sek() -> HourlyResult<Hourly> {
    from_url(NORDPOOL_URL_SEK)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ColEntry {
    Index: u8,
    Scale: u8,
    SecondaryValue: Option<String>,
    IsDominatingDirection: bool,
    IsValid: bool,
    IsAdditionalData: bool,
    Behavior: u8,
    Name: String,
    Value: String,
    GroupHeader: Option<String>,
    DisplayNegativeValueInBlue: bool,
    CombinedName: String,
    DateTimeForData: NaiveDateTime,
    DisplayName: String,
    DisplayNameOrDominatingDirection: String,
    IsOfficial: bool,
    UseDashDisplayStyle: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RowEntry {
    Columns: Vec<ColEntry>,
    Name: String,
    StartTime: NaiveDateTime,
    EndTime: NaiveDateTime,
    DateTimeForData: NaiveDateTime,
    DayNumber: u16,
    StartTimeDate: NaiveDateTime,
    IsExtraRow: bool,
    IsNtcRow: bool,
    EmptyValue: String,
    Parent: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    Rows: Vec<RowEntry>,
    DataStartdate: NaiveDateTime,
    DataEnddate: NaiveDateTime,
    MinDateForTimeScale: NaiveDateTime,
    DateUpdated: NaiveDateTime,
    LatestResultDate: NaiveDateTime,
    ContainsPreliminaryValues: bool,
    ContainsExchangeRates: bool,
    CombinedHeadersEnabled: bool,
    DataType: i16,
    TimeZoneInformation: i16,
    Units: Vec<String>,
    IsDivided: bool,
}

// This enum "Currencies" is only used to satisfy nordpools raw price data JSON structure..
// We never use it ourselves because currency is stored within the data entry in the field "units".
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    DKK,
    EUR,
    NOK,
    SEK,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // re-format the name "pageId" from input data to "page_id" used in the struct.
pub struct Hourly {
    data: Data,
    currency: Currency,
    page_id: usize,

    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl Hourly {
    fn new(json_str: &str) -> HourlyResult<Self> {
        match serde_json::from_str::<Self>(json_str) {
            Ok(data) => {
                // Page id for hourly elspot is 10.
                if data.page_id != 10 {
                    return Err(HourlyError::InvalidPageID);
                }

                // Check if 'unit_string' is in the array.
                if data.data.Units.is_empty() {
                    panic!("No unit string in data.Units");
                }

                // Test 'unit_string' to ensure it is valid.
                let unit_string = &data.data.Units[0];
                if let Err(e) = units::test_unit_string(unit_string) {
                    panic!("{}, found value '{}' in data.Units[0]", e, unit_string);
                }

                Ok(data)
            }
            Err(_) => {
                Err(HourlyError::InvalidJSON)
            }
        }
    }

    fn col_index_for_region(&self, region: &str) -> RegionResult<usize> {
        let columns = &self.data.Rows[0].Columns;

        let res: Option<&ColEntry> = columns
            .iter()
            .find(|v| v.Name == region);

        match res {
            Some(v) => Ok(v.Index.into()),
            None => Err(RegionError::RegionIndexNotFound),
        }
    }

    /// Check if prices are official (if not, then prices might change).
    pub fn prices_are_official(&self) -> bool {
        for row in self.data.Rows.iter() {
            for col in row.Columns.iter() {
                if !col.IsOfficial {
                    return false;
                }
            }
        }

        true
    }
    /// Prints all available `regions` in the price dataset.
    pub fn print_regions(&self) {
        println!("Available regions:");
        for col in &self.data.Rows[0].Columns {
            println!("'{}' ", col.Name);
        }
        println!();
    }

    /// Returns a Vec<&str> of all available `regions` in the price dataset.
    pub fn regions(&self) -> Vec<&str> {
        self.data.Rows[0].Columns
            .iter()
            .map(|col| col.Name.as_ref())
            .collect()
    }

    pub fn has_region(&self, region: &str) -> bool {
        let columns = &self.data.Rows[0].Columns;

        let res: Option<&ColEntry> = columns
            .iter()
            .find(|v| v.Name == region);

        res.is_some()
    }

    pub fn date(&self) -> NaiveDate {
        self.data.DataStartdate.date()
    }

    pub fn extract_prices_for_region(&self, region: &str) -> Vec<Price> {
        let index_for_region = self.col_index_for_region(region).unwrap_or_else(|e| panic!("{}", e));

        let _prices: Vec<&ColEntry> = self.data.Rows
            .iter()
            .filter(|&row| !&row.IsExtraRow && &row.Columns[index_for_region].Value != "-")
            .map(|row| &row.Columns[index_for_region])
            .collect();

        if _prices.is_empty() {
            return vec![];
        }

        let price_hours = PriceHours::new(self.date(), region);
        let unit_string = &self.data.Units[0];
        let mut start_time: DateTime<Tz> = dt_tz_from_naive_dt(self.data.Rows[0].StartTime, region);
        let mut end_time: DateTime<Tz> = dt_tz_from_naive_dt(self.data.Rows[0].EndTime, region);
        let mut prices_region: Vec<Price> = vec![];
        match price_hours {
            PriceHours::TwentyThree => assert_eq!(price_hours.as_int(), _prices.len()),
            PriceHours::TwentyFive => assert_eq!(price_hours.as_int(), _prices.len()),
            // PriceHours::TwentyFour if _prices.len() != 24 => assert_eq!(_prices.len(), 25),
            _ => assert_eq!(_prices.len() as u8, 24),
        };

        for price in _prices {
            if region != "SYS" {
                assert_eq!(self.date(), start_time.date_naive());
            }

            let p = Price {
                value: price.Value.to_string(),
                from: start_time.to_utc(),
                to: end_time.to_utc(),
                region: region.to_string(),
                currency_unit: units::Currency::new(unit_string).unwrap_or_else(|e| panic!("{}", e)),
                power_unit: units::Power::new(unit_string).unwrap_or_else(|e| panic!("{}", e)),
            };

            prices_region.push(p);

            start_time += Duration::hours(1);
            end_time += Duration::hours(1);
        }

        prices_region
    }

    pub fn extract_all_prices(&self) -> Vec<Vec<Price>> {
        let mut prices_regions: Vec<Vec<Price>> = vec![];
        for region in self.regions() {
            prices_regions.push(self.extract_prices_for_region(region));
        }

        prices_regions
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn to_file(&self, path: &str) {
        let s = self.to_string();
        fs::write(path, s.as_bytes()).unwrap_or_else(|e| panic!("{}", e));
    }
}

impl fmt::Display for Hourly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
