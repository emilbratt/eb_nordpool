use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    EUR,
    DKK,
    NOK,
    PLN,
    RON,
    SEK,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const SUPPORTED_CURRENCIES: [&str; 6] = [
    "EUR",
    "DKK",
    "NOK",
    "PLN",
    "RON",
    "SEK",
];

pub fn list_supported() {
    println!("Supported currencies");
    for c in SUPPORTED_CURRENCIES {
        println!("'{}' ", c);
    }
    println!();
}
