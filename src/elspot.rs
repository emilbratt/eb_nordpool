use std::fmt;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub mod hourly;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currencies {
    DKK,
    EUR,
    NOK,
    SEK,
}

impl fmt::Display for Currencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Prices returned comes in the form of this datatype.
#[derive(Clone, Debug)]
pub struct Price {
    pub region: String,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub value: String,
    pub unit: String, // e.g. "NOK/MWh".
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Price {
    pub fn currency_unit(&self) -> String {
        todo!()
        // let i = self.unit.find('/').unwrap();
        // self.unit.get(..i).unwrap().to_string()
    }

    pub fn power_unit(&self) -> String {
        todo!()
        // let i = self.unit.find('/').unwrap() + 1;
        // self.unit.get(i..).unwrap().to_string()
    }

    pub fn unit_format(&self) -> String {
        todo!()
        // for example "Eurocent/MWh"
    }

    pub fn price_human(&self) -> String {
        todo!()
        // for example:
        //      "16 ore/kWh"
        //      "18 EUR/MWh"
        //..
    }

    pub fn price_as_float() -> f32 {
        todo!()
    }
}
