//! `eb_nordpool` provides an easy way to extract elspot prices from Nordpool.
//!
//! # Working with data from https://www.nordpoolgroup.com/api/marketdata/page/10
//!
//! NOTE: This api is removed, but you might want to work with data already stored on your drive.
//!
//! ```
//! // You need to import these modules.
//! use eb_nordpool::{
//!     elspot::marketdata_page_10,
//!     error::ElspotError,
//!     units,
//! };
//!
//! // Date for prices, formatted as "YYYY-MM-DD" (chrono's NaiveDate type).
//! data.date()
//!
//! // Save data to file.
//! data.to_file("path/to/data.json");
//!
//! // Load data from local file.
//! let data = marketdata_page_10::from_file("path/to/data.json").unwrap();
//!
//! // Load data from http server.
//! let data = marketdata_page_10::from_url("http.....").unwrap();
//!
//! // Serialize data to json string, nice if you want to load it somewhere else.
//! let s = data.to_json_string();
//!
//! // Print out all available regions. This is convenient for finding a specific region.
//! data.print_regions();
//!
//! // All available regions in a Vec<&str>.
//! let regions = data.regions();
//!
//! // Check if region exists in dataset.
//! if data.has_region("Oslo") {
//!     // ..do something
//! }
//!
//! ```
//!
//! Extracting prices.
//!
//! ```
//! // Get all prices for specific region (always in time ascending order starting at 00:00).
//! let prices = data.extract_prices_for_region("Oslo");
//!
//! // Get time window (from and to) for a price in chrono's datetime type.
//! let p = &prices[0];
//! // DateTime with the same timezone as the region for the prices.
//! let (from, to) = p.from_to(); // (from, to) as (DateTime<Tz>, DateTime<Tz>)
//! // Adjusted for Utc.
//! let (from_utc, to_utc) = p.from_to_utc(); // (from, to) as (DateTime<Utc>, DateTime<Utc>)
//! // Adjusted for region, for example Finland using region code "FI".
//! let (from_r, to_r) = p.from_to_region("FI"); // (from, to) as (DateTime<Tz>, DateTime<Tz>)
//! // Adjusted for any timezone, for example Los Angeles using chrono_tz's tz type.
//! use chrono_tz::America::Los_Angeles;
//! let (from_la, to_la) = p.from_to_tz(Los_Angeles);
//!
//! // Convert to other units (includes changing price value to accommodate for new units).
//! let mut prices = data.extract_prices_for_region("Oslo");
//! for mut p in prices.iter_mut() {
//!     // To fractional currency unit (from for example 'Kr.' to 'Øre').
//!     units::convert_to_currency_fraction(&mut p);
//!     // To smaller power unit (from "MWh" to "kWh").
//!     units::convert_to_kwh(&mut p);
//!
//!     // And back again.
//!     // units::convert_to_currency_full(&mut p);
//!     // units::convert_to_mwh(&mut p);
//! }
//! for p in prices.iter() {
//!     assert!(p.currency_unit.is_fraction());
//!     assert!(p.power_unit.is_kwh());
//!     println!("{}", p.price_label());
//! }
//!
//! // Pretty print price (label like). Looks like this: "NOK 167,68 Kr./MWh".
//! let p = &prices[8];
//! println!("{}", p.price_label());
//!
//! // Get price as numeric data types.
//! let p = &prices[8];
//! let f: f32 = p.as_f32();
//! let i: i32 = p.as_i32();
//!
//! // Just get all prices for all regions in a 2D Array.
//! let regions = data.extract_all_prices();
//! for region in regions.iter() {
//!     for price in region.iter() {
//!         println!("Time: {} - {} ({})", price.hour(), price.price_label(), price.region)
//!     }
//! }
//! ```

#![allow(non_snake_case)] // Struct naming is in "PascalCase" to map directly with data from nordpool..
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_same_arms)]
#![allow(missing_docs)]

pub mod elspot;
pub mod error;
pub mod region_time;
pub mod units;
