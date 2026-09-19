use std::{error::Error, io, str::FromStr};

use rust_decimal::Decimal;
use serde::Deserialize;

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
pub fn load_yahoo_daily_closes(path: &str) -> Result<Vec<Decimal>, Box<dyn Error>> {
    let file_contents = std::fs::read_to_string(path)?;
    let parsed: YahooChartFile = serde_json::from_str(&file_contents)?;
    let closes = &parsed
        .chart
        .result
        .first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Yahoo result is empty"))?
        .indicators
        .quote
        .first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Yahoo quote is empty"))?
        .close;

    closes
        .iter()
        .flatten()
        .map(|price| Decimal::try_from(*price).map_err(Into::into))
        .collect()
}

/// FRED writes "." on non-trading days, so values that will not parse are
/// skipped rather than coerced to zero.
pub fn load_fred_daily_values(path: &str) -> Result<Vec<Decimal>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut values = Vec::new();

    for record in reader.records() {
        if let Some(value) = record?.get(1).and_then(|text| Decimal::from_str(text).ok()) {
            values.push(value);
        }
    }

    Ok(values)
}
