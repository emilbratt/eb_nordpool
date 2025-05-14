# EB NordPool - An elspot fetching Library 🦀

* Fetch elspot prices from NordPool with Rust

### Getting Started

```rust
use eb_nordpool::{
    elspot::from_nordpool,
    elspot::dataportal_dayaheadprices::{regions, currencies},
    units,
};

// Select date in "YYYY-MM-DD" format (NOTE: only last two months).
let date = "2025-05-14";
// Select currency.
let currency = "EUR";
// Select regions.
let _regions = vec!["NO1", "SE3", "FI"]; // Must have at least one.
// or select all regions..
let regions = regions::SUPPORTED_REGIONS.iter().map(|r| r.as_ref()).collect::<Vec<&str>>();

// NOTE: you can easily print out all supported currencies and regions..
currencies::list_supported();
regions::list_supported();

// Now we can download data.
let data = from_nordpool(currency, date, &regions).unwrap();

// And now we can extract the prices and do stuff.
let mut regions = data.extract_prices_all_regions();
for prices in regions.iter_mut() {
    for p in prices.into_iter() {
        let (from, to) = p.from_to();
        println!("{} | From: {from} - To: {to}", p.region);

        println!("{}", p.price_label());

        // Convert to more sane units.
        units::convert_to_kwh(p);
        units::convert_to_currency_fraction(p);

        println!("{}\n", p.price_label());

        // Get price as float.
        let _p = p.as_f64();
    }
}

println!("date: {}", data.date());
println!("currency: {}", data.currency());
println!("is final: {}", data.is_final());
```

Refer to [crate docs] for more details on how to fetch prices.

[crate docs]: https://docs.rs/eb_nordpool/latest/eb_nordpool/

### Testing

For all but the "ignored" ones found in download.rs.

```sh
cargo test

```

For ignored ones in download.rs (uses no capture because we need to read output data).

```sh
cargo test -- --ignored --nocapture
```
