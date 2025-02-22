use std::{fs, fmt};

use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    Duration,
};
use chrono_tz::Tz;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::error::{
    ElspotError,
    ElspotResult,
};
use crate::region_time::dt_tz_from_naive_dt;
use crate::units;

use super::{PriceExtractor, Price};

mod hour_count;
mod unit_string;

use hour_count::HoursForDate;

pub fn from_url(url: &str) -> ElspotResult<PriceData> {
    let r = reqwest::blocking::get(url).unwrap();

    let json_str = r.text().unwrap();

    PriceData::new(&json_str)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ColEntry {
    Index: u8,
    IsOfficial: bool,
    Name: String,
    Value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RowEntry {
    Columns: Vec<ColEntry>,
    EndTime: NaiveDateTime,
    IsExtraRow: bool,
    StartTime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    ContainsPreliminaryValues: bool,
    DataStartdate: NaiveDateTime,
    Rows: Vec<RowEntry>,
    Units: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // re-format the name "pageId" from input data to "page_id" used in the struct.
pub struct PriceData {
    data: Data,
    currency: String,
    page_id: usize,
}

impl PriceExtractor for PriceData {
    fn new(json_str: &str) -> ElspotResult<Self> {
        match serde_json::from_str::<Self>(json_str) {
            Ok(data) => {
                // Page id for hourly elspot is 10.
                if data.page_id != 10 {
                    return Err(ElspotError::MarketdataPage10InvalidPageId);
                }

                // Check if 'unit_string' is in the array.
                if data.data.Units.is_empty() {
                    return Err(ElspotError::MarketdataPage10MissingUnitString);
                }

                // Test 'unit_string' to ensure it is valid.
                let unit_string = &data.data.Units[0];
                if let Err(e) = unit_string::test_unit_string(unit_string) {
                    panic!("{}: '{}'", e, unit_string);
                }

                Ok(data)
            }
            Err(_e) => {
                // FIXME: show this error if user wants to..
                // eprintln!("{_e}");
                Err(ElspotError::MarketdataPage10InvalidJson)
            }
        }
    }

    /// Check if prices are final.
    fn is_final(&self) -> bool {
        !self.data.ContainsPreliminaryValues
    }

    /// Check if prices are not finite.
    fn is_preliminary(&self) -> bool {
        self.data.ContainsPreliminaryValues
    }

    /// Prints all available `regions` in the price dataset.
    fn print_regions(&self) {
        println!("Available regions:");
        for col in &self.data.Rows[0].Columns {
            println!("'{}' ", col.Name);
        }
        println!();
    }

    /// Returns a Vec<&str> of all available `regions` in the price dataset.
    fn regions(&self) -> Vec<&str> {
        self.data.Rows[0].Columns
            .iter()
            .map(|col| col.Name.as_ref())
            .collect()
    }

    /// Check if region exist in dataset.
    fn has_region(&self, region: &str) -> bool {
        let columns = &self.data.Rows[0].Columns;

        let res: Option<&ColEntry> = columns
            .iter()
            .find(|v| v.Name == region);

        res.is_some()
    }

    fn currency(&self) -> String {
        self.currency.clone()
    }

    fn date(&self) -> NaiveDate {
        self.data.DataStartdate.date()
    }

    fn extract_prices_for_region(&self, region: &str) -> Vec<Price> {
        // Verify that the amount of prices extracted matches the amount of hours in that particular day.
        let index: usize = match self.data.Rows[0].Columns.iter().find(|col| col.Name == region) {
            None => {
                eprintln!("Prices for {region} not found.");
                return vec![];
            }
            Some(entry) => entry.Index.into(),
        };
        let raw_prices: Vec<&ColEntry> = self.data.Rows
            .iter()
            .filter(|row| !row.IsExtraRow && row.Columns[index].Value != "-")
            .map(|row| &row.Columns[index])
            .collect();
        if raw_prices.is_empty() {
            // no prices where found, that is ok..
            return vec![];
        }
        let hours_for_date = HoursForDate::new(self.date(), region);
        let verified = match raw_prices.len() {
            23 => matches!(hours_for_date, HoursForDate::TwentyThree),
            25 => matches!(hours_for_date, HoursForDate::TwentyFive),
            24 => matches!(hours_for_date, HoursForDate::TwentyFour),
            _ => false,
        };
        if !verified {
            eprintln!("FATAL: Non matching price count for {region}.");
            eprintln!("Got '{}' prices, expected 23, 24 or 25", raw_prices.len());
            eprintln!("Dumping data: 'raw_prices'\n{:?}", raw_prices);
            panic!();
        }

        // Now we can start assembling the real price data.
        let cur_unit = unit_string::extract_currency_unit(&self.data.Units[0]);
        let e_cur_unit = units::Currency::new(cur_unit).unwrap_or_else(|e| panic!("{}", e));

        let pwr_unit = unit_string::extract_power_unit(&self.data.Units[0]);
        let e_pwr_unit = units::Power::new(pwr_unit).unwrap_or_else(|e| panic!("{}", e));

        // Always 60 minutes for this old nordpool api.
        let mtu = units::Mtu::Sixty;

        let mut start_time: DateTime<Tz> = dt_tz_from_naive_dt(self.data.Rows[0].StartTime, region);
        let mut end_time: DateTime<Tz> = dt_tz_from_naive_dt(self.data.Rows[0].EndTime, region);

        let mut extr_prices: Vec<Price> = Vec::with_capacity(raw_prices.len());

        for price in raw_prices {
            if region != "SYS" {
                assert_eq!(self.date(), start_time.date_naive());
            }

            let p = Price {
                value: price.Value.to_string().replace(',', ".").replace(' ', ""),
                from: start_time.to_utc(),
                to: end_time.to_utc(),
                date: self.data.DataStartdate.date(),
                region: region.to_string(),
                currency_unit: e_cur_unit.clone(),
                market_time_unit: mtu,
                power_unit: e_pwr_unit.clone(),
            };

            extr_prices.push(p);

            start_time += Duration::hours(1);
            end_time += Duration::hours(1);
        }

        extr_prices
    }

    fn extract_prices_all_regions(&self) -> Vec<Vec<Price>> {
        let mut prices_all: Vec<Vec<Price>> = vec![];
        for region in self.regions() {
            prices_all.push(self.extract_prices_for_region(region));
        }

        prices_all
    }

    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|e| panic!("{}", e))
    }

    fn to_file(&self, path: &str) {
        let s = serde_json::to_string(&self).unwrap_or_else(|e| panic!("{}", e));
        fs::write(path, s.as_bytes()).unwrap_or_else(|e| panic!("{}", e));
    }
}

impl fmt::Display for PriceData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
