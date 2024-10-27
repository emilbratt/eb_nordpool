use std::{env, fs, fmt};

use crate::debug::Debug;
use crate::elspot::{PriceExtractor, Price};
use crate::error::{
    ElspotError,
    ElspotResult,
};
use crate::region_time::dt_tz_from_naive_dt;
use crate::units;

use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    Duration,
};
use chrono_tz::Tz;

use serde::{Deserialize, Serialize};
use serde_json;

mod hour_count;
mod unit_string;

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
            Err(e) => {
                if env::var("EB_NORDPOOL_DEBUG").is_ok() {
                    let file = "elspot/marketdata_page_10.rs";
                    let msg = format!("serde_json: {}", e);
                    Debug::new(file, &msg).print();
                }
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
        let res: Option<&ColEntry> = self.data.Rows[0].Columns
            .iter()
            .find(|v| v.Name == region);

        let index_for_region: usize = match res {
            Some(v) => v.Index.into(),
            None => panic!(),
        };

        // Extract all price values into a simple vector, exluding non-price values.
        let _prices: Vec<&ColEntry> = self.data.Rows
            .iter()
            .filter(|&row| !&row.IsExtraRow && &row.Columns[index_for_region].Value != "-")
            .map(|row| &row.Columns[index_for_region])
            .collect();


        let hour_count = hour_count::Hours::new(self.date(), region);

        // Verify that the amount of prices extracted matches the hours in a day.
        match _prices.len() {
            0 => return vec![], // no prices where found, that is ok.
            23 => assert!(matches!(hour_count, hour_count::Hours::TwentyThree)),
            25 => assert!(matches!(hour_count, hour_count::Hours::TwentyFive)),
            24 => assert!(matches!(hour_count, hour_count::Hours::TwentyFour)),
            n => panic!("Weird price count: {}", n),
        }

        // Now we can start assembling the real price data.
        let cur_unit = unit_string::extract_currency_unit(&self.data.Units[0]);
        let e_cur_unit = units::Currency::new(cur_unit).unwrap_or_else(|e| panic!("{}", e));

        let pwr_unit = unit_string::extract_power_unit(&self.data.Units[0]);
        let e_pwr_unit = units::Power::new(pwr_unit).unwrap_or_else(|e| panic!("{}", e));

        let mtu = units::Mtu::Sixty; // Is always 60 minutes for this nordpool api.

        let mut start_time: DateTime<Tz> = dt_tz_from_naive_dt(self.data.Rows[0].StartTime, region);
        let mut end_time: DateTime<Tz> = dt_tz_from_naive_dt(self.data.Rows[0].EndTime, region);

        let mut prices: Vec<Price> = vec![];

        for price in _prices {
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

            prices.push(p);

            start_time += Duration::hours(1);
            end_time += Duration::hours(1);
        }

        prices
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
