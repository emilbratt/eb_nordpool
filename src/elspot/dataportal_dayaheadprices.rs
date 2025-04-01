use std::{fs, fmt};
use std::collections::HashMap;

use chrono::{
    Utc,
    DateTime,
    NaiveDate,
};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::error::{
    ElspotError,
    ElspotResult,
};
use crate::units;

use super::{PriceExtractor, Price};

pub mod currencies;
pub mod regions;
pub mod query;

mod states;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AreaAverage {
    area_code: regions::Region,
    price: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AreaState {
    state: states::State,
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
pub struct PriceData {
    delivery_date_c_e_t: NaiveDate,
    version: u8,
    delivery_areas: Vec<String>,
    market: String,
    multi_area_entries: Vec<AreaEntries>,
    block_price_aggregates: Vec<PriceAggregates>,
    currency: currencies::Currency,
    exchange_rate: f32,
    area_states: Vec<AreaState>,
    area_averages: Vec<AreaAverage>,
}

impl PriceExtractor for PriceData {
    fn new(json_str: &str) -> ElspotResult<Self> {
        match serde_json::from_str::<Self>(json_str) {
            Ok(data) => {
                // FIXME:: do we need these code blocks below?
                // if data.version < 1 || data.version > 3 {
                //     return Err(ElspotError::DataPortalDayaheadPricesInvalidVersion);
                // }

                // if data.market != "DayAhead" {
                //     return Err(ElspotError::DataPortalDayaheadPricesInvalidMarket);
                // }

                Ok(data)
            }
            Err(_e) => {
                // FIXME: show this error if user wants to..
                // eprintln!("{_e}");
                Err(ElspotError::DataPortalDayaheadPricesInvalidJson)
            }
        }
    }

    /// Check if prices are final.
    fn is_final(&self) -> bool {
        for states in self.area_states.iter() {
            if !states.state.is_final() {
                return false;
            }
        }

        true
    }

    /// Prices are either final or preliminary.
    fn is_preliminary(&self) -> bool {
        for states in self.area_states.iter() {
            if states.state.is_preliminary() {
                return true;
            }
        }

        false
    }

    /// Prints all available `regions` in the price dataset.
    fn print_regions(&self) {
        println!("Available regions:");
        for r in self.multi_area_entries[0].entry_per_area.iter() {
            println!("{}", r.0);
        }
        println!();

    }

    /// Returns a vector of all available `regions` in the price dataset.
    fn regions(&self) -> Vec<&str> {
        self.multi_area_entries[0].entry_per_area
            .iter()
            .map(|v| v.0.as_ref())
            .collect()
    }

    /// Check if region exist in dataset.
    fn has_region(&self, region: &str) -> bool {
        for r in self.multi_area_entries.iter() {
            if !r.entry_per_area.contains_key(region) {
                return false;
            }
        }

        true
    }

    fn currency(&self) -> String {
        self.currency.to_string()
    }

    fn date(&self) -> NaiveDate {
        self.delivery_date_c_e_t
    }

    fn extract_prices_for_region(&self, region: &str) -> Vec<Price> {
        if !self.has_region(region) {
            return vec![];
        }

        let mut prices: Vec<Price> = Vec::with_capacity(100);
        for e in self.multi_area_entries.iter() {
            let v = e.entry_per_area[region].to_string();

            let cu = units::Currency::new(&self.currency.to_string()).unwrap_or_else(|e| panic!("{}", e));
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

    fn extract_prices_all_regions(&self) -> Vec<Vec<Price>> {
        let mut prices_all: Vec<Vec<Price>> = vec![];
        for region in self.delivery_areas.iter() {
            let prices = self.extract_prices_for_region(region);
            if !prices.is_empty() {
                prices_all.push(prices);
            }
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
