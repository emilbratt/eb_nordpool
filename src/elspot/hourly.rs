use std::fs;

use crate::elspot::{self, Price};
use crate::error::{Error, Result};
use crate::region_time;

use chrono::{
    DateTime,
    NaiveDateTime,
    Timelike,
    Utc,
    Duration
};
use chrono_tz::Tz;

use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};

use reqwest;

pub fn from_json(json_str: &str) -> Result<Hourly> {
    let hourly = Hourly::new(json_str)?;

    Ok(hourly)
}

pub fn from_file(path: &str) -> Result<Hourly> {
    let json_str = fs::read_to_string(path).unwrap();

    from_json(&json_str)
}

pub fn from_url(url: &str) -> Result<Hourly> {
    let body = reqwest::blocking::get(url).unwrap()
        .text().unwrap();

    from_json(&body)
}

pub fn from_nordpool(currency: elspot::Currencies) -> Result<Hourly> {
    match currency {
        elspot::Currencies::DKK => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=DKK"),
        elspot::Currencies::EUR => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=EUR"),
        elspot::Currencies::NOK => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=NOK"),
        elspot::Currencies::SEK => from_url("https://www.nordpoolgroup.com/api/marketdata/page/10?currency=SEK"),
    }
}

#[derive(Serialize, Debug, Clone, Deserialize)]
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

#[derive(Serialize, Debug, Clone, Deserialize)]
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

#[derive(Serialize, Debug, Clone, Deserialize)]
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

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")] // re-format the name "pageId" from input data to "page_id" used in the struct.
pub struct Hourly {
    data: Data,
    currency: elspot::Currencies,
    page_id: u8,

    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl Hourly {
    fn new(json_str: &str) -> Result<Self> {
        let d: Self = serde_json::from_str(json_str).unwrap();

        match d.page_id {
            10 => Ok(d),
            _ => Err(Error::HourlyInvalidPageID),
        }
    }

    fn col_index_for_region(&self, region: &str) -> usize {
        let columns = &self.data.Rows[0].Columns;

        let res: Option<&ColEntry> = columns
            .iter()
            .find(|v| v.Name == region);

        match res {
            Some(v) => v.Index.into(),
            None => panic!("{:?}", Error::RegionIndexNotFound),
        }
    }

    pub fn list_regions(&self) {
        println!("Available regions:");
        for col in self.data.Rows[0].Columns.iter() {
            print!("'{}' ", col.Name);
        }
        println!();
    }

    pub fn prices_are_for_today(&self, region: &str) -> bool {
        self.data.DataStartdate.date() == region_time::local_dt_now_from_region(region).date_naive()
    }

    pub fn has_region(&self, region: &str) -> bool {
        let columns = &self.data.Rows[0].Columns;

        let res: Option<&ColEntry> = columns
            .iter()
            .find(|v| v.Name == region);

        res.is_some()
    }

    pub fn price_for_region_at_utc_dt(&self, region: &str, utc_dt: &DateTime<Utc>) -> Result<Price> {
        let region_dt: DateTime<Tz> = region_time::region_dt_from_utc_dt(region, utc_dt);

        let region_index = self.col_index_for_region(region);

        if self.data.DataStartdate.date() != region_dt.date_naive() {
            return Err(Error::HourlyPriceDateMismatch);
        }

        let row_entries: Vec<&RowEntry> = self.data.Rows
            .iter()
            .filter(|v| !v.IsExtraRow && v.StartTime.hour() == region_dt.hour())
            .collect();

        // Find the row holding the prices for our selected time (all regions are included here).
        let row_entry: &RowEntry;

        match row_entries.len() {
            0 => return Err(Error::HourlyPriceHourMismatch),
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
                    _ => return Err(Error::HourlyPriceHourMismatchCESTToCET),
                }
            },
            _ => return Err(Error::HourlyPriceFilteredRowsExceededTwo),
        }

        let p = Price {
            value: row_entry.Columns[region_index].Value.to_string(),
            time: row_entry.StartTime,
            unit: self.data.Units[0].to_string(),
            region: region.to_string(),
        };

        Ok(p)
    }

    pub fn price_now_for_region(&self, region: &str) -> Result<Price> {
        let utc_now = Utc::now();
        self.price_for_region_at_utc_dt(region, &utc_now)
    }

    pub fn all_prices_for_region(&self, region: &str) -> Vec<Price> {
        let mut prices: Vec<Price> = vec![];

        let region_index = self.col_index_for_region(region);

        for row in self.data.Rows.iter() {
            if row.IsExtraRow {
                continue; // Extra rows are reserved for aggregate values such as min, max, avg etc..
            } else if row.StartTime.hour() == 2 && row.Columns[region_index].Value == "-" {
                continue; // Moving from CET to CEST and the nordpool data includes an empty value, but we skip it.
            } else {
                let p = Price {
                    value: row.Columns[region_index].Value.to_string(),
                    time: row.StartTime,
                    unit: self.data.Units[0].to_string(),
                    region: region.to_string(),
                };

                prices.push(p)
            }
        }

        prices
    }
}
