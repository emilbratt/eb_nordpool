#![allow(unused)]

use std::{fs, fmt};
use std::collections::HashMap;

use crate::elspot::Price;
use crate::error::{
    ElspotError,
    ElspotResult,
    RegionError,
    RegionResult,
};

use crate::units;

use chrono::{
    Local,
    Utc,
    DateTime,
    NaiveDate,
    NaiveDateTime,
    Duration,
};
use chrono_tz::Tz;

use serde::{Deserialize, Serialize};
use serde_json;

use reqwest;

pub mod regions;
pub mod query;

pub fn from_json(json_str: &str) -> ElspotResult<MarkedData> {
    MarkedData::new(json_str)
}

pub fn from_file(path: &str) -> ElspotResult<MarkedData> {
    let json_str = fs::read_to_string(path).unwrap();

    from_json(&json_str)
}

pub fn from_url(url: &str) -> ElspotResult<MarkedData> {
    let r = reqwest::blocking::get(url).unwrap();

    let json_str = r.text().unwrap();

    from_json(&json_str)
}

pub fn from_nordpool(date: &str, regions: &Vec<&str>, currency: &str) -> ElspotResult<MarkedData> {
    let naive_date: NaiveDate = match date {
        "today" => Local::now().date_naive(),
        "tomorrow" => (Local::now() + Duration::days(1)).date_naive(),
        naive_date => NaiveDate::parse_from_str(naive_date,"%Y-%m-%d").unwrap(),
    };

    let date = &naive_date.to_string();

    let q = query::QueryOptions::new(currency, date, regions);
    from_url(&q.build())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AreaAverage {
    area_code: regions::Region,
    price: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum State {
    Preliminary, // This variant might be wrong, must find out later..
    Final,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AreaState {
    state: State,
    areas: Vec<regions::Region>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Aggregate {
    average: f32,
    min: f32,
    max: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AreaEntries {
    delivery_start: DateTime<Utc>,
    delivery_end: DateTime<Utc>,
    entry_per_area: HashMap<String, f32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct PriceAggregates {
    block_name: String,
    delivery_start: DateTime<Utc>,
    delivery_end: DateTime<Utc>,
    average_price_per_area: HashMap<String, Aggregate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkedData {
    delivery_date_c_e_t: NaiveDate,
    version: u8,
    delivery_areas: Vec<String>,
    market: String,
    multi_area_entries: Vec<AreaEntries>,
    block_price_aggregates: Vec<PriceAggregates>,
    currency: String,
    exchange_rate: f32,
    area_states: Vec<AreaState>,
    area_averages: Vec<AreaAverage>,
}

impl MarkedData {
    pub fn new(json_str: &str) -> ElspotResult<Self> {
        match serde_json::from_str::<Self>(json_str) {
            Ok(data) => {
                if data.version != 3 {
                    return Err(ElspotError::DataPortalDayaheadPricesInvalidVersion);
                }

                if data.market != "DayAhead" {
                    return Err(ElspotError::DataPortalDayaheadPricesInvalidMarket);
                }

                Ok(data)
            }
            Err(e) => {
                println!("{}", e);
                Err(ElspotError::MarketdataPage10InvalidJson)
            }
        }
    }

    /// Prices are either final or preliminary.
    pub fn is_preliminary(&self) -> bool {
        for states in self.area_states.iter() {
            if matches!(states.state, State::Preliminary) {
                return true;
            }
        }

        false
    }

    /// Prints all available `regions` in the price dataset.
    pub fn print_regions(&self) {
        println!("Available regions:");
        for r in self.delivery_areas.iter() {
            println!("'{}' ", r);
        }
        println!();
    }

    /// Returns a Vec<&str> of all available `regions` in the price dataset.
    pub fn regions(&self) -> Vec<&str> {
        self.delivery_areas
            .iter()
            .map(|v| v.as_ref())
            .collect()
    }

    pub fn has_region(&self, region: &str) -> bool {
        if let Err(e) = regions::Region::from_str(region) {
            panic!("{}: '{}' is not a supported region", e, region);
        }

        for r in self.delivery_areas.iter() {
            if r == region {
                return true;
            }
        }

        false
    }

    pub fn currency(&self) -> &str {
        self.currency.as_ref()
    }

    pub fn date(&self) -> NaiveDate {
        self.delivery_date_c_e_t
    }

    pub fn extract_prices_for_region(&self, region: &str) -> Vec<Price> {
        if !self.has_region(region) {
            panic!("'{}' cound not be found", region);
        }

        let mut prices: Vec<Price> = vec![];

        for e in self.multi_area_entries.iter() {
            let v = e.entry_per_area[region].to_string();

            let cu = units::Currency::new(&self.currency).unwrap_or_else(|e| panic!("{}", e));
            let pu = units::Power::new("MWh").unwrap_or_else(|e| panic!("{}", e));
            let mtu = units::Mtu::new(e.delivery_start, e.delivery_end).unwrap_or_else(|e| panic!("{}", e));

            let p = Price {
                value: v,
                from: e.delivery_start,
                to: e.delivery_end,
                date: self.delivery_date_c_e_t,
                region: region.to_string(),
                currency_unit: cu,
                market_time_unit: mtu,
                power_unit: pu,
            };

            prices.push(p);
        }

        prices
    }

    pub fn extract_prices_all_regions(&self) -> Vec<Vec<Price>> {
        let mut prices_regions: Vec<Vec<Price>> = vec![];
        for region in self.delivery_areas.iter() {
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

impl fmt::Display for MarkedData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
