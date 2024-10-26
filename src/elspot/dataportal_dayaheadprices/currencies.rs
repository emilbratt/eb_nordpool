use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    EUR,
    DKK,
    NOK,
    SEK,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const SUPPORTED_CURRENCIES: [&str; 4] = [
    "EUR",
    "DKK",
    "NOK",
    "SEK",
];

pub fn print_all() {
    println!("Supported currencies");
    for c in SUPPORTED_CURRENCIES {
        println!("'{}' ", c);
    }
    println!();
}
