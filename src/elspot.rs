use std::fmt;

use chrono::{DateTime, Utc, NaiveDate};
use chrono_tz::Tz;

use crate::region_time::dt_region_from_utc_dt;
use crate::units;

pub mod dataportal_dayaheadprices;
pub mod marketdata_page_10;

/// Each price returned comes in the form of this datatype.
#[derive(Clone, Debug)]
pub struct Price {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    pub date: NaiveDate,
    pub region: String,
    pub value: String,
    pub currency_unit: units::Currency,
    pub market_time_unit: units::Mtu,
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

        // Test number before starting.
        self.value.parse::<f32>().unwrap_or_else(|e| panic!("{}: '{}' could not be parsad into float", e, self.value));

        let mut split = self.value.split('.');
        let whole_numbers = split.next().unwrap();
        match split.next() {
            Some("") | None => {
                whole_numbers.parse::<f32>().unwrap()
            }
            Some(fractions) => {
                let formatted: String = if fractions.len() > 3 {
                    // Only keep at most 3 fractions, the 3rd is for rounding and fixes some rounding errors.
                    format!("{}.{}", whole_numbers, &fractions.to_string()[..3])
                } else {
                    format!("{}.{}", whole_numbers, fractions)
                };

                let f32_parsed = formatted.parse::<f32>().unwrap();

                // Only keep two decimal places fractions.
                let f32_two_decimals = (f32_parsed * 100.0).round() / 100.0;

                if self.currency_unit.is_fraction() {
                    // Currency sub-unit does not use fractions, we round all the way up.
                    f32_two_decimals.round()
                } else {
                    f32_two_decimals
                }
            }
        }
    }

    pub fn as_i32(&self) -> i32 {
        self.as_f32().round() as i32
    }

    pub fn hour(&self) -> String {
        self.to.format("%H:%M").to_string()
    }

    pub fn price_label(&self) -> String {
        let value = self.as_f32().to_string().replace('.', ",");
        let currency_unit = self.currency_unit.to_string();
        let power_unit = self.power_unit.to_string();
        let country = self.currency_unit.country_code();

        format!("{country} {value} {currency_unit}/{power_unit}")
    }

    pub fn from_to(&self) -> (DateTime<Tz>, DateTime<Tz>) {
        (dt_region_from_utc_dt(&self.from, &self.region), dt_region_from_utc_dt(&self.to, &self.region))
    }

    pub fn from_to_as_utc(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        (self.from, self.to)
    }

    pub fn from_to_with_region(&self, region: &str) -> (DateTime<Tz>, DateTime<Tz>) {
        (dt_region_from_utc_dt(&self.from, region), dt_region_from_utc_dt(&self.to, region))
    }

    pub fn from_to_with_tz(&self, tz: Tz) -> (DateTime<Tz>, DateTime<Tz>) {
        (self.from.with_timezone(&tz), self.to.with_timezone(&tz))
    }
}
