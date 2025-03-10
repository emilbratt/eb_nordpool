use eb_nordpool::elspot;

#[test]
fn eur_24h() {
    // testing standard 24h days data.
    let data = elspot::from_file("./tests/data/marketdata_page_10_EUR_24H.json").unwrap();
    assert_eq!("EUR", data.currency());
    assert_eq!("2024-06-20", data.date().to_string());
    assert!(!data.is_preliminary());
    assert!(data.has_region("Tr.heim"));
    assert!(data.has_region("SE1"));
    assert!(data.has_region("FI"));
    assert!(data.has_region("BE"));

    let prices = data.extract_prices_for_region("Tr.heim");
    let p = &prices[1];
    assert_eq!("5.82", p.value);
    assert_eq!(data.date(), p.date);

    let prices = data.extract_prices_for_region("FI");
    let p = &prices[2];
    assert_eq!("-5.00", p.value);
    assert_eq!(data.date(), p.date);

    let prices_all = data.extract_prices_all_regions();
    for prices in prices_all {
        assert_eq!(prices.len(), 24);
        for p in prices {
            let (from, _) = p.from_to();
            assert_eq!(data.date(), from.date_naive());
        }
    }
}

#[test]
fn nok_25h() {
    // Test when we have 25 hours in a day.
    let data = elspot::from_file("./tests/data/marketdata_page_10_NOK_25H.json").unwrap();
    assert!(data.is_preliminary());

    let price = data.extract_prices_for_region("Tr.heim");
    let p = &price[3];
    assert_eq!("167.66", p.value);

    for region in data.regions() {
        match region {
            // Test data has no prices for these regions..
            "AT" | "BE" | "DE-LU" | "FR" |"NL" => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 0);
            },
            _ => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 25);
                if region != "SYS" {
                    for p in prices {
                        let (from, _) = p.from_to();
                        assert_eq!(data.date(), from.date_naive());
                    }
                }
             }
        }
    }
}

#[test]
fn nok_23h() {
    // Test when we have 23 hours in a day.
    let data = elspot::from_file("./tests/data/marketdata_page_10_NOK_23H.json").unwrap();
    assert!(data.is_preliminary());
    assert_eq!("2023-03-26", data.date().to_string());

    let prices = data.extract_prices_for_region("Oslo");
    assert_eq!(prices.len(), 23);
    let p = &prices[0];
    let (from, to) = p.from_to();
    assert_eq!(from.to_rfc3339(), "2023-03-26T00:00:00+01:00");
    assert_eq!(to.to_rfc3339(), "2023-03-26T01:00:00+01:00");

    let data = elspot::from_file("./tests/data/marketdata_page_10_NOK_23H.json").unwrap();
    for region in data.regions() {
        match region {
            // Test data has no prices for these regions..
            "AT" | "BE" | "DE-LU" | "FR" |"NL" => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 0);
            },
            _ => {
                let prices = data.extract_prices_for_region(region);
                assert_eq!(prices.len(), 23);
                for p in prices {
                    let (from, _) = p.from_to();
                    assert_eq!(data.date(), from.date_naive());
                }
            }
        }
    }
}

#[test]
fn to_json_string() {
    let data = elspot::from_file("./tests/data/marketdata_page_10_EUR_24H.json").unwrap();

    // Save data to a string (serialized json).
    let s = data.to_json_string();

    // We just reload the string and see if it works, unwrap() will fail if Err.
    elspot::from_json(&s).unwrap();
}
