use std::fs;

use crate::elspot::{self, Price};
use crate::error::{
    HourlyError,
    HourlyResult,
    RegionError,
    RegionResult,
};
use crate::region_time;

use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    Timelike,
    Utc,
    Duration
};
use chrono_tz::Tz;

use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};

use reqwest;

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

pub fn from_nordpool(currency: elspot::Currencies) -> HourlyResult<Hourly> {
    match currency {
        elspot::Currencies::DKK => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=DKK"),
        elspot::Currencies::EUR => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=EUR"),
        elspot::Currencies::NOK => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=NOK"),
        elspot::Currencies::SEK => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=SEK"),
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // re-format the name "pageId" from input data to "page_id" used in the struct.
pub struct Hourly {
    data: Data,
    currency: elspot::Currencies,
    page_id: u8,

    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl Hourly {
    fn new(json_str: &str) -> HourlyResult<Self> {
        let d = serde_json::from_str::<Self>(json_str);

        match d {
            Ok(v) => {
                match v.page_id {
                    10 => Ok(v),
                    _ => Err(HourlyError::InvalidPageID),
                }
            },
            Err(_) => {
                Err(HourlyError::InvalidJSON)
            },
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

    /// Prints out a list of all `regions` found in the price data and how they are spelled.
    ///
    /// <pre>
    /// Available regions:
    /// 'SYS'
    /// 'SE1'
    /// 'SE2'
    /// 'SE3'
    /// 'SE4'
    /// 'FI'
    /// 'DK1'
    /// 'DK2'
    /// 'Oslo'
    /// ...
    /// </pre>
    pub fn print_regions(&self) {
        println!("Available regions:");
        for col in self.data.Rows[0].Columns.iter() {
            println!("'{}' ", col.Name);
        }
        println!();
    }

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

    pub fn prices_are_today_for_region(&self, region: &str) -> bool {
        self.date() == region_time::region_dt_now_from_region(region).date_naive()
    }

    pub fn price_for_region_at_utc_dt(&self, region: &str, utc_dt: &DateTime<Utc>) -> HourlyResult<Price> {
        let region_dt: DateTime<Tz> = region_time::region_dt_from_utc_dt(region, utc_dt);

        let index_for_region = self.col_index_for_region(region).unwrap_or_else(|e| panic!("{}", e));

        if self.data.DataStartdate.date() != region_dt.date_naive() {
            return Err(HourlyError::PriceDateMismatch);
        }

        let row_entries: Vec<&RowEntry> = self.data.Rows
            .iter()
            .filter(|v| !v.IsExtraRow && v.StartTime.hour() == region_dt.hour())
            .collect();

        // Find the row holding the prices for our selected time (all regions are included here).
        let row_entry: &RowEntry;

        match row_entries.len() {
            0 => return Err(HourlyError::PriceHourMismatch),
            1 => {
                // 99.9% of the time, this block will match because we only have one entry for a specific time.
                row_entry = row_entries[0];
            },
            2 => {
                // This happens on rare occasions.
                // The datetime moves from CEST to CET e.g. from summer time to winter time.
                // This means that we have a 25 hour day AND the hour now must be 2 o'clock since we have two entries..
                // What happens is we end up with two possible times (2 o'clock CEST and 2 o'clock CET).
                // To disambiguate we add 1 hour to the datetime and see if this moves us to 3 o'clock.
                // If 2 then we are still on CEST, if 3 we are on CET.
                match (region_dt + Duration::hours(1)).hour() {
                    3 => row_entry = row_entries[1],
                    2 => row_entry = row_entries[0],
                    _ => return Err(HourlyError::PriceHourMismatchCESTToCET),
                }
            },
            _ => return Err(HourlyError::PriceFilteredRowsExceededTwo),
        }

        let p = Price {
            value: row_entry.Columns[index_for_region].Value.to_string(),
            from: row_entry.StartTime,
            to: row_entry.EndTime,
            unit: self.data.Units[0].to_string(),
            region: region.to_string(),
        };

        Ok(p)
    }

    pub fn price_now_for_region(&self, region: &str) -> HourlyResult<Price> {
        let utc_now = Utc::now();
        self.price_for_region_at_utc_dt(region, &utc_now)
    }

    pub fn all_prices_for_region(&self, region: &str) -> Vec<Price> {
        let mut prices: Vec<Price> = vec![];

        let index_for_region = self.col_index_for_region(region).unwrap_or_else(|e| panic!("{}", e));

        for row in self.data.Rows.iter() {
            if row.IsExtraRow {
                continue; // Extra rows are reserved for aggregate values such as min, max, avg etc..
            } else if row.StartTime.hour() == 2 && row.Columns[index_for_region].Value == "-" {
                continue; // Moving from CET to CEST and the nordpool data includes an empty value which we skip.
            } else {
                let p = Price {
                    value: row.Columns[index_for_region].Value.to_string(),
                    from: row.StartTime,
                    to: row.EndTime,
                    unit: self.data.Units[0].to_string(),
                    region: region.to_string(),
                };

                prices.push(p);
            }
        }

        prices
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn to_file(&self, path: &str) {
        let s = self.to_string();
        fs::write(path, s.as_bytes()).unwrap_or_else(|e| panic!("{}", e));
    }
}
