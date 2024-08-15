//! eb_nordpool provides an easy way to extract elspot prices from Nordpool.
//!
//! # Getting started
//!
//! Fetching prices from nordpoolgroup.com or from file.
//!
//! ```
//! use eb_nordpool::{elspot, region_time};
//!
//! // Set the currency "DKK, EUR, NOK or SEK", when downloading prices from nordpool.
//! let currency = elspot::Currencies::NOK;
//!
//! // Download todays or tomorrows prices (we can not control this..)
//! // If time is before 13:00 in Norway, you get prices for today; else you get for tomorrow.
//! let data = elspot::hourly::from_nordpool(currency).unwrap();
//! // ..this http request is currently blocking, but this might change in the future.
//!
//! // Gives you the actual date for the prices in YYYY-MM-DD format (chrono's NaiveDate type).
//! data.date()
//!
//! // Save data to file.
//! data.to_file("path/to/data.json");
//!
//! // When or if you have data stored locally, you can simply load it from a file.
//! let data = elspot::hourly::from_file("path/to/data.json").unwrap();
//!
//! // Serialize data to json string, nice if you want to load it somewhere else.
//! let s = data.to_string();
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
//! let (region, from, to, value, unit) = (&p.region, &p.from, &p.to, &p.value, &p.unit);
//! println!("Price for {region} between {from} and {to} is {value} {unit}");
//!
//! // Price now for specific region.
//! if let Ok(p) = hourly.price_now_for_region("Oslo") {
//!     println!("Price for Oslo now:  {} {}", p.value, p.unit);
//! }
//!
//! // Get price for specific timestamp (must be in Utc)
//! let utc_dt = region_time::utc_with_ymd_and_hms(2024, 6, 20, 11, 0, 0);
//! let p = hourly.price_for_region_at_utc_dt("Oslo", &utc_dt);
//! // NOTE: this gives you the price for 13:00 local time, Oslo is 2 hours ahead during CEST..
//! match p {
//!     Ok(p) => println!("{}", p),
//!     Err(e) => println!("{}", e),
//! }
//! ```

#![allow(non_snake_case)] // Struct naming is in "PascalCase" to map directly with data from nordpool..

pub mod elspot;
pub mod error;
pub mod region_time;
