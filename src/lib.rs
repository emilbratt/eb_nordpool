//! `eb_nordpool` provides an easy way to extract elspot prices from Nordpool.

//! # Working with data from dataportal-api.nordpoolgroup.com/api/DayAheadPrices
//!
//! ```
//! use eb_nordpool::{
//!     elspot::dataportal_dayaheadprices,
//!     error::ElspotError,
//!     units,
//! };
//!
//! // Load data from local file.
//! let data = dataportal_dayaheadprices::from_file("path/to/data.json").unwrap();
//!
//! // Load data from json string.
//! let data = dataportal_dayaheadprices::from_json("{..}").unwrap();
//!
//! // Load data directly from nordpool.
//! // Select date in "YYYY-MM-DD" format.
//! let date = "2024-10-24";
//! // Select currency.
//! let currency = "NOK";
//! // Select regions.
//! let regions = vec!["NO1", "SE3", "FI"]; // Must have at least one.
//! // or select all regions..
//! let mut regions: Vec<&str> = vec![];
//! for region in dataportal_dayaheadprices::regions::SUPPORTED_REGIONS.iter() {
//!     regions.push(region);
//! }
//! // NOTE: you can easily print out all supported currencies and regions..
//! dataportal_dayaheadprices::currencies::print_all();
//! dataportal_dayaheadprices::regions::print_all();
//! // Finally, download data.
//! let data = dataportal_dayaheadprices::from_nordpool(currency, date, regions).unwrap();
//! ```

//! # Working with data from api/marketdata/page/10
//!
//! NOTE: This api is removed, but you might want to work with data already stored on your drive.
//!
//! ```
//! use eb_nordpool::{
//!     elspot::marketdata_page_10,
//!     error::ElspotError,
//!     units,
//! };
//!
//! // Load data from local file.
//! let data = marketdata_page_10::from_file("path/to/data.json").unwrap();
//!
//! // Load data from json string.
//! let data = marketdata_page_10::from_json("{..}").unwrap();
//!
//! // Load data from http server.
//! let data = marketdata_page_10::from_url("http..").unwrap();
//! ```

//! # Extracting the data (and more stuff..)
//!
//! ```
//! // Once you have loaded the data with one of the two workflows above, we can do stuff.
//!
//! // Get date for prices, formatted as "YYYY-MM-DD" (chrono's NaiveDate type).
//! data.date()
//! 
//! // Print out all available regions.
//! data.print_regions();
//!
//! // Get all available regions in a Vec<&str>.
//! let regions = data.regions();
//!
//! // Check if region exists in dataset.
//! if data.has_region("Oslo") {
//!     // ..do something
//! }
//!
//! // Get all prices for specific region (always in time ascending order starting at 00:00).
//! let prices = data.extract_prices_for_region("Oslo");
//!
//! // Get time window (from and to) for a price in chrono's datetime type.
//! let p = &prices[0];
//! // Adjusted for same timezone as the region for the prices.
//! let (from, to) = p.from_to();
//! // Adjusted for Utc.
//! let (from_utc, to_utc) = p.from_to_as_utc();
//! // Adjusted for region, for example Finland using region code "FI".
//! let (from_r, to_r) = p.from_to_with_region("FI");
//! // Adjusted for any timezone, for example Los Angeles using chrono_tz's tz type.
//! use chrono_tz::America::Los_Angeles;
//! let (from_la, to_la) = p.from_to_with_tz(Los_Angeles);
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
//! let f = p.as_f32();
//! let i = p.as_i32();
//!
//! // Just get all prices for all regions in a 2D Array.
//! let regions = data.extract_prices_all_regions();
//! for prices in regions.iter() {
//!     for price in prices.iter() {
//!         println!("Time: {} - {} ({})", price.hour(), price.price_label(), price.region)
//!     }
//! }
//!
//! // Save data to file.
//! data.to_file("save/to/data.json");
//!
//! // Serialize data to json string.
//! let s = data.to_json_string();
//!
//! ```

#![allow(non_snake_case)] // Struct naming is in "PascalCase" to map directly with data from nordpool..
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_same_arms)]
#![allow(missing_docs)]

pub mod elspot;
pub mod error;
pub mod region_time;
pub mod units;
