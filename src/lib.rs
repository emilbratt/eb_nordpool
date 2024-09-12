//! `eb_nordpool` provides an easy way to extract elspot prices from Nordpool.
//!
//! # Getting started
//!
//! Fetching prices from nordpoolgroup.com or from file.
//!
//! ```
//! use eb_nordpool::{elspot, region_time, units};
//!
//! // Download todays or tomorrows prices (we can not control this..)
//! // If time is before 13:00 in Norway, you get prices for today; else you get for tomorrow.
//! let data = elspot::hourly::from_nordpool_nok().unwrap();
//! // ..this http request is currently blocking, but this might change in the future.
//!
//! // Gives you the actual date for the prices in YYYY-MM-DD format (chrono's NaiveDate type).
//! data.date()
//!
//! // Save data to file.
//! data.to_file("path/to/data.json");
//!
//! // Load data from tile.
//! let data = elspot::hourly::from_file("path/to/data.json").unwrap();
//!
//! // Serialize data to json string, nice if you want to load it somewhere else.
//! let s = data.to_json_string();
//!
//! // Print out all available regions. This is convenient for finding a specific region.
//! data.print_regions();
//!
//! // Get all regions in a Vec<&str>, nice if you want to do something with all regions.
//! let regions = data.regions();
//!
//! // Check if a region exists in dataset.
//! if data.has_region("Oslo") {
//!     // ..do something
//! }
//!
//! // Price now for specific region.
//! if let Ok(p) = data.price_now_for_region("Oslo") {
//!     println!("Price for Oslo now: {}", p.price_label());
//! }
//!
//! // Get price for specific timestamp (must be in Utc)
//! let utc_dt = region_time::utc_with_ymd_and_hms(2024, 6, 20, 11, 0, 0);
//! let p = data.price_for_region_at_utc_dt("Oslo", &utc_dt);
//! // NOTE: this gives you the price for 13:00 local time (Oslo is 2 hours ahead during CEST..).
//! match p {
//!     Ok(p) => println!("{}", p),
//!     Err(e) => println!("{}", e),
//! }
//!
//! // Get all prices for a specific region (always in ascending order starting at time 00:00).
//! let prices = data.all_prices_for_region("Oslo");
//!
//! // Print all price data.
//! for p in prices.iter() {
//!     println!("{}", p);
//! }
//!
//! // Print one price.
//! let p = &prices[8];
//! let (region, from, to, label) = (&p.region, &p.from, &p.to, &p.price_label());
//! println!("Price for {region} between {from} and {to} is {label}");
//!
//! // Pretty print price (label like). Looks like this: "NOK 167,68 Kr./MWh".
//! let p = &prices[8];
//! println!("{}", p.price_label());
//!
//! // Get price as f32.
//! let p = &prices[8];
//! let p_as_float = p.as_f32();
//!
//! // Convert currency-units and power-units.
//! let mut p = prices[3].clone();
//! units::convert_to_currency_fraction(&mut p); // Converts "160,00" to "16000" e.g. to cents.
//! units::convert_to_currency_full(&mut p); // Same as above, but the other way around.
//! units::convert_to_kwh(&mut p); // Converts from MWh to kWh (also adjusts the price value).
//! units::convert_to_mwh(&mut p); // Same as above, but the other way around.
//! ```

#![allow(non_snake_case)] // Struct naming is in "PascalCase" to map directly with data from nordpool..
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_same_arms)]
#![allow(missing_docs)]

pub mod elspot;
pub mod error;
pub mod region_time;
pub mod units;
