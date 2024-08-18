//! Keep in mind that we only operate on String values and not numeric values like ints and floats.
//! When converting a currency to its sub-unit or when we adjust a price for power-unit MWh to kWh, we move the comma.
//! The reason for this is to preserve the number precision that could otherwise get lost in decimal operations.
//! This is especially true when doing the division operation.

use crate::elspot::Price;

// Swaps the currency unit string back and forth between full-unit and sub-unit,
// NOTE: a cent is indicated with the 1% in parentheses.
fn currency_unit_swap(unit: &mut String) {
    // Unit format:
    // "Cur/Pwr"
    //
    // Example:
    // "NOK/MWh"
    //
    // The dots are changed in this function.
    // ".../MWh"
    if let Some(i) = unit.find('/') {
        match unit.get(..i) {
            Some("EUR") => unit.replace_range(..i, "EUR(1%)"),
            Some("EUR(1%)") => unit.replace_range(..i, "EUR"),

            Some("NOK") => unit.replace_range(..i, "NOK(1%)"),
            Some("NOK(1%)")  => unit.replace_range(..i, "NOK"),

            Some("DKK") => unit.replace_range(..i, "DKK(1%)"),
            Some("DKK(1%)")  => unit.replace_range(..i, "DKK"),

            Some("SEK") => unit.replace_range(..i, "SEK(1%)"),
            Some("SEK(1%)")  => unit.replace_range(..i, "SEK"),

            Some(v) => panic!("Invalid currency unit '{}'", v),
            None => panic!(),
        }
    } else {
        panic!("Could not find the currency and power-unit separator '/' in the unit string..");
    }
}

fn power_unit_swap(unit: &mut String) {
    // Unit format:
    // "Cur/Pwr"
    //
    // Example:
    // "NOK/MWh"
    //
    // The dots are changed in this function.
    // "NOK/..."
    let i = unit.len() - 3; // By convenience, the offset we want is always located here.
    match unit.get(i..unit.len()) {
        Some("MWh") => unit.replace_range(i.., "kWh"),
        Some("kWh") => unit.replace_range(i.., "MWh"),

        Some(v) => panic!("Invalid power unit '{}' or wrong offset..", v),
        None => panic!("No power unit found, is the offset correct?"),
    }
}

fn move_comma_right(value: &mut String, moves: usize) {
    // Remove leading zeros.
    while value.chars().next() == Some('0') {
        value.remove(0);
    }

    match value.find(',') {
        Some(i) => {
            // Remove trailing zeros as they have no value after comma.
            while value.chars().last() == Some('0') {
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
    while value.chars().next() == Some('0') {
        value.remove(0);
    }

    match value.find(',') {
        Some(i) => {
            value.remove(i);
            if i <= moves {
                value.insert_str(0, "0".repeat(moves-i).as_ref());
                value.insert_str(0, "0,");
            } else {
                value.insert_str(i-moves, ",");
            }

            // Remove trailing zeros as they have no value as last digit after comma.
            while value.chars().last() == Some('0') {
                value.pop();
            }

            // Remove trailing comma if there are no fractions left.
            if value.chars().last() == Some(',') {
                value.pop();
            }
        },
        None => {
            while value.len() <= moves {
                value.insert(0, '0');
            }

            value.insert_str(value.len()-moves, ",");
        },
    }
}


/// Converts the base currency to its fractional value by moving comma 2 steps to the right.
pub fn to_currency_sub_unit(p: &mut Price) {
    if let Some(_) = p.unit.find("(1%)") {
        panic!("Currency already in sub-unit '{}'", p.unit);
    }

    move_comma_right(&mut p.value, 2);
    currency_unit_swap(&mut p.unit);
}

/// Converts the currencies sub-unit to its full value by moving comma 2 steps to the left.
pub fn to_currency_full_unit(p: &mut Price) {
    if let None = p.unit.find("(1%)") {
        panic!("Currency already in full unit '{}'", p.unit);
    }

    move_comma_left(&mut p.value, 2);
    currency_unit_swap(&mut p.unit);
}

/// The price is calculated to 1/1000 of its original value (1/1000M = 1k).
pub fn to_power_kwh_unit(p: &mut Price) {
    if let Some(_) = p.unit.find("kWh") {
        panic!("Power unit already in kWh '{}'", p.unit);
    }

    move_comma_left(&mut p.value, 3);
    power_unit_swap(&mut p.unit);
}

/// The price is calculated to 1000x of its original value (1000k = 1M).
pub fn to_power_mwh_unit(p: &mut Price) {
    if let Some(_) = p.unit.find("MWh") {
        panic!("Power unit already in MWh '{}'", p.unit);
    }

    move_comma_right(&mut p.value, 3);
    power_unit_swap(&mut p.unit);
}
