use chrono::NaiveDate;
use url::Url;

use crate::elspot::dataportal_dayaheadprices::currencies::SUPPORTED_CURRENCIES;
use crate::elspot::dataportal_dayaheadprices::regions::SUPPORTED_REGIONS;

const NORDPOOL_BASE_URL: &str = "https://dataportal-api.nordpoolgroup.com/api/DayAheadPrices";

pub struct QueryOptions<'a> {
    currency: Option<&'a str>,
    date: Option<&'a str>,
    regions: Vec<&'a str>,
}

impl <'a>QueryOptions<'a> {
    pub fn new() -> Self {
        Self {
            currency: None,
            date: None,
            regions: vec![],
        }
    }

    pub fn set_currency(&mut self, currency: &'a str) {
        if !SUPPORTED_CURRENCIES.contains(&currency) {
            let supported = SUPPORTED_CURRENCIES
                .iter()
                .map(|v| format!("\n{}", v))
                .collect::<String>();

            panic!("'{}' is not a supported currency, use one of:{}", currency, supported);
        }
        self.currency = Some(currency);
    }

    pub fn set_date(&mut self, date: &'a str) {
        if let Err(e) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            panic!("{}: '{}' is not a valid date, use the format '%Y-%m-%d' for example: '2018-01-26'", e, date);
        }

        self.date = Some(date);
    }

    pub fn set_regions(&mut self, regions: Vec<&'a str>) {
        for region in regions.iter() {
            if !SUPPORTED_REGIONS.contains(&region) {
                let supported = SUPPORTED_REGIONS
                    .iter()
                    .map(|v| format!("{}\n", v))
                    .collect::<String>();

                panic!("'{}' is not a supported region, use any of the regions listed below\n{}", region, supported);
            }
            if !self.regions.contains(region) {
                self.regions.push(region);
            }
        }
    }

    pub fn build_url(&self) -> String {
        let date = match self.date {
            None => panic!("No date was set, use .set_date(date) to set a specific date"),
            Some(date) => date,
        };

        let currency = match self.currency {
            None => panic!("No currency was set, use .set_currency(currency) to set a specific currency"),
            Some(currency) => currency,
        };

        if self.regions.len() == 0 {
            panic!("No regions set, use .add_region(region) to add 1 or more regions");
        }

        let mut regions = String::from(self.regions[0]);
        for i in 1..self.regions.len() {
            regions.push_str(format!(",{}", self.regions[i]).as_ref());
        }

        let mut url = Url::parse(NORDPOOL_BASE_URL).unwrap();
        url.query_pairs_mut().append_pair("currency", currency);
        url.query_pairs_mut().append_pair("date", date);
        url.query_pairs_mut().append_pair("market", "DayAhead");
        url.query_pairs_mut().append_pair("deliveryArea", regions.as_ref());

        url.as_str().to_string()
    }
}
