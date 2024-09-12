//! Keep in mind that we only operate on String values and not numeric values like ints and floats.
//! When converting a currency to its sub-unit or when we adjust a price for power-unit MWh to kWh, we move the comma.
//! The reason for this is to preserve the number precision that could otherwise get lost in decimal operations.
//! This is especially true when doing the division operation.

use std::fmt;

use crate::elspot::Price;
use crate::error::{
    UnitError,
    UnitResult,
};

// The unit string is found somewhere inside the data-set from nordpool.
pub const EXPECTED_UNIT_SRINGS: [&str; 4] = [
    "EUR/MWh",
    "DKK/MWh",
    "NOK/MWh",
    "SEK/MWh",
];

#[derive(Clone, Debug)]
pub enum CurrencyUnit {
    Full, // main currency unit such as "Euro (EUR)", "Kroner (NOK)" etc..
    Fraction, // sub currency unit such as "Cent (EUR)", "øre (NOK)" etc..
}

#[derive(Clone, Debug)]
pub enum Currency {
    EUR(CurrencyUnit),
    DKK(CurrencyUnit),
    NOK(CurrencyUnit),
    SEK(CurrencyUnit),
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Currency {
    pub fn new(unit_string: &str) -> UnitResult<Self> {
        // unit_string looks like this "EUR/MWh"..
        let currency_unit = &unit_string[..3];
        if currency_unit == "EUR" {
            Ok(Self::EUR(CurrencyUnit::Full))
        } else if currency_unit == "DKK" {
            Ok(Self::DKK(CurrencyUnit::Full))
        } else if currency_unit == "NOK" {
            Ok(Self::NOK(CurrencyUnit::Full))
        } else if currency_unit == "SEK" {
            Ok(Self::SEK(CurrencyUnit::Full))
        } else {
            Err(UnitError::InvalidCurrencyUnit)
        }
    }

    fn to_fraction(&mut self) {
        *self = match self {
            Self::EUR(_) => Self::EUR(CurrencyUnit::Fraction),
            Self::DKK(_) => Self::DKK(CurrencyUnit::Fraction),
            Self::NOK(_) => Self::NOK(CurrencyUnit::Fraction),
            Self::SEK(_) => Self::SEK(CurrencyUnit::Fraction),
        };
    }

    fn to_full(&mut self) {
        *self = match self {
            Self::EUR(_) => Self::EUR(CurrencyUnit::Full),
            Self::DKK(_) => Self::DKK(CurrencyUnit::Full),
            Self::NOK(_) => Self::NOK(CurrencyUnit::Full),
            Self::SEK(_) => Self::SEK(CurrencyUnit::Full),
        };
    }

    pub fn is_fraction(&self) -> bool {
        match self {
            Self::EUR(CurrencyUnit::Fraction) => true,
            Self::DKK(CurrencyUnit::Fraction) => true,
            Self::NOK(CurrencyUnit::Fraction) => true,
            Self::SEK(CurrencyUnit::Fraction) => true,

            Self::EUR(CurrencyUnit::Full) => false,
            Self::DKK(CurrencyUnit::Full) => false,
            Self::NOK(CurrencyUnit::Full) => false,
            Self::SEK(CurrencyUnit::Full) => false,
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            Self::EUR(CurrencyUnit::Full) => true,
            Self::DKK(CurrencyUnit::Full) => true,
            Self::NOK(CurrencyUnit::Full) => true,
            Self::SEK(CurrencyUnit::Full) => true,

            Self::EUR(CurrencyUnit::Fraction) => false,
            Self::DKK(CurrencyUnit::Fraction) => false,
            Self::NOK(CurrencyUnit::Fraction) => false,
            Self::SEK(CurrencyUnit::Fraction) => false,
        }
    }

    pub fn country_str(&self) -> String {
        match self {
            Self::EUR(_) => String::from("EUR"),
            Self::DKK(_) => String::from("DKK"),
            Self::NOK(_) => String::from("NOK"),
            Self::SEK(_) => String::from("SEK"),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Self::EUR(CurrencyUnit::Full) => String::from("Eur."),
            Self::EUR(CurrencyUnit::Fraction) => String::from("Cent"),

            Self::DKK(CurrencyUnit::Full) => String::from("Kr."),
            Self::DKK(CurrencyUnit::Fraction) => String::from("Øre"),

            Self::NOK(CurrencyUnit::Full) => String::from("Kr."),
            Self::NOK(CurrencyUnit::Fraction) => String::from("Øre"),

            Self::SEK(CurrencyUnit::Full) => String::from("Kr."),
            Self::SEK(CurrencyUnit::Fraction) => String::from("öre"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum Power {
    MWh,
    kWh,
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Power {
    pub fn new(unit_string: &str) -> UnitResult<Self> {
        // unit_string looks like this "EUR/MWh"..
        let power_unit = &unit_string[4..];
        if power_unit == "MWh" {
            Ok(Self::MWh)
        } else if power_unit == "kWh" {
            Ok(Self::kWh)
        } else {
            Err(UnitError::InvalidPowerUnit)
        }
    }

    fn is_kwh(&self) -> bool {
        match self {
            Self::MWh => false,
            Self::kWh => true,
        }
    }

    fn to_kwh(&mut self) {
        *self = Self::kWh;
    }

    fn to_mwh(&mut self) {
        *self = Self::MWh;
    }

    fn is_mwh(&self) -> bool {
        match self {
            Self::MWh => true,
            Self::kWh => false,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Self::MWh => String::from("MWh"),
            Self::kWh => String::from("kWh"),
        }
    }
}

fn move_comma_right(value: &mut String, moves: usize) {
    // Remove leading zeros.
    while value.starts_with('0') {
        value.remove(0);
    }

    match value.find(',') {
        Some(i) => {
            // Remove trailing zeros as they have no value after comma.
            while value.ends_with('0') {
                value.pop();
            }
            value.remove(i);

            let fractions = value.len() - i;
            if fractions < moves {
                value.push_str("0".repeat(moves-fractions).as_ref());
            }

            // Insert comma if it will not end up as the last character.
            if i+moves < value.len() {
                value.insert(i+moves, ',');
            }
        },
        None => {
            value.push_str("0".repeat(moves).as_ref());
        },
    }
}

fn move_comma_left(value: &mut String, moves: usize) {
    // Remove leading zeros.
    while value.starts_with('0') {
        value.remove(0);
    }

    if let Some(i) = value.find(',') {
        value.remove(i);
        if i <= moves {
            value.insert_str(0, "0".repeat(moves-i).as_ref());
            value.insert_str(0, "0,");
        } else {
            value.insert(i-moves, ',');
        }

        // Remove trailing zeros as they have no value as last digit after comma.
        while value.ends_with('0') {
            value.pop();
        }

        // Remove trailing comma if there are no fractions left.
        if value.ends_with(',') {
            value.pop();
        }
    } else {
        while value.len() <= moves {
            value.insert(0, '0');
        }

        value.insert(value.len()-moves, ',');
    }
}

/// Converts the base currency to its fractional value by moving comma 2 steps to the right.
pub fn convert_to_currency_fraction(p: &mut Price) {
    if p.currency_unit.is_fraction() {
        panic!("Currency already fraction unit '{}'", p.currency_unit.to_str());
    }

    move_comma_right(&mut p.value, 2);
    p.currency_unit.to_fraction();
}

/// Converts the currencies sub-unit to its full value by moving comma 2 steps to the left.
pub fn convert_to_currency_full(p: &mut Price) {
    if p.currency_unit.is_full() {
        panic!("Currency already full unit '{}'", p.currency_unit.to_str());
    }

    move_comma_left(&mut p.value, 2);
    p.currency_unit.to_full();
}

/// The price is calculated to 1/1000 of its original value (1/1000M = 1k).
pub fn convert_to_kwh(p: &mut Price) {
    if p.power_unit.is_kwh() {
        panic!("Power unit already kWh");
    }

    move_comma_left(&mut p.value, 3);
    p.power_unit.to_kwh();
}

/// The price is calculated to 1000x of its original value (1000k = 1M).
pub fn convert_to_mwh(p: &mut Price) {
    if p.power_unit.is_mwh() {
        panic!("Power unit already MWh");
    }

    move_comma_right(&mut p.value, 3);
    p.power_unit.to_mwh();
}

pub fn _test_unit_result(b: bool) -> UnitResult<()> {
    match b {
        true => Ok(()),
        false => Err(UnitError::InvalidCurrencyUnit),
    }
}
