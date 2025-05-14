### eb_nordpool

Rust library for extracting elspot prices from nordpool.

[Documentation](https://docs.rs/eb_nordpool/latest/eb_nordpool/)

### Testing

For all but the "from_nordpool" in download.rs

```sh
cargo test
```

For "from_nordpool" in download.rs (uses no capture because we need to read output data).

```sh
cargo test from_nordpool -- --nocapture
```
