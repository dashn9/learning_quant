use rust_decimal::Decimal;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct YahooChartFile {
    chart: YahooChart,
}

#[derive(Deserialize)]
struct YahooChart {
    result: Vec<YahooResult>,
}

#[derive(Deserialize)]
struct YahooResult {
    indicators: YahooIndicators,
}

#[derive(Deserialize)]
struct YahooIndicators {
    quote: Vec<YahooQuote>,
}

#[derive(Deserialize)]
struct YahooQuote {
    close: Vec<Option<f64>>,
}

/// Yahoo reports prices as f64 and writes null on days with no bar.
/// Nulls are dropped so a missing day never masquerades as a zero price.
pub fn load_yahoo_daily_closes(path: &str) -> Vec<Decimal> {
    let file_contents = std::fs::read_to_string(path).unwrap();
    let parsed: YahooChartFile = serde_json::from_str(&file_contents).unwrap();

    parsed.chart.result[0].indicators.quote[0]
        .close
        .iter()
        .flatten()
        .map(|price| Decimal::try_from(*price).unwrap())
        .collect()
}

/// FRED writes "." on non-trading days, so values that will not parse are
/// skipped rather than coerced to zero.
pub fn load_fred_daily_values(path: &str) -> Vec<Decimal> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut values = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();
        if let Ok(value) = Decimal::from_str(&record[1]) {
            values.push(value);
        }
    }

    values
}
