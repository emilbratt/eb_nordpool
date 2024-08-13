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
//! // Already have the data stored locally? You can simply load from a file instead.
//! let data = elspot::hourly::from_file("path/to/data.json").unwrap();
//!
//! // Print out all available regions. This is convenient for finding a specific region.
//! data.list_regions();
//!
//! // Get current date for prices in the YYYY-MM-DD format.
//! data.date()
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
//! println!("Price for {} between {} and {} is {} {} ", p.region, p.from, p.to, p.value, p.unit);
//!
//! // Price now for specific region.
//! if let Ok(p) = hourly.price_now_for_region("Oslo") {
//!     println!("Price for Oslo is currenlty {} {}", p.value, p.unit);
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
