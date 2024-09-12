use std::fmt;
use chrono::NaiveDateTime;

use crate::units;

pub mod hourly;

/// Prices returned comes in the form of this datatype.
#[derive(Clone, Debug)]
pub struct Price {
    pub region: String,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub value: String,
    pub currency_unit: units::Currency,
    pub power_unit: units::Power,
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Price {
    pub fn as_f32(&self) -> f32 {
        // This function will try its best to round the floating point number to the correct value.
        // Large numbers (including negative) or numbers with many fractional digits,
        // might in rare cases be rounded the wrong way due to floating point precision errors.
        let f = self.value.replace(',', ".");

        // Test number before starting.
        assert!(f.parse::<f32>().is_ok());

        let mut split = f.split('.');
        let whole_numbers = split.next().unwrap();
        match split.next() {
            Some("") | None => {
                whole_numbers.parse::<f32>().unwrap()
            }
            Some(fractions) => {
                let formatted: String = if fractions.len() > 3 {
                    // Only keep at most 3 fractions, the 3rd is for rounding and fixes some bugs with rounding errors.
                    format!("{}.{}", whole_numbers, &fractions.to_string()[..3])
                } else {
                    format!("{}.{}", whole_numbers, fractions)
                };

                let f32_parsed = formatted.parse::<f32>().unwrap();
                let f32_two_decimals = (f32_parsed * 100.0).round() / 100.0; // Only keep two decimal places fractions.
                if self.currency_unit.is_fraction() {
                    f32_two_decimals.round() // Currency sub-unit does not use fractions, we round all the way up.
                } else {
                    f32_two_decimals
                }
            }
        }
    }

    pub fn price_label(&self) -> String {
        let value = self.as_f32().to_string().replace('.', ",");
        let currency_unit = self.currency_unit.to_str();
        let power_unit = self.power_unit.to_str();
        let country = self.currency_unit.country_str();

        format!("{country} {value} {currency_unit}/{power_unit}")
    }
}
