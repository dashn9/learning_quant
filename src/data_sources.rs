use chrono::{DateTime, Utc};
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
    timestamp: Vec<i64>,
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

pub struct YahooDailyClose {
    pub date: String,
    pub closing_price: Decimal,
}

/// Yahoo reports prices as f64 and writes null on days with no bar.
/// Nulls are dropped so a missing day never masquerades as a zero price.
pub fn load_yahoo_daily_closes(path: &str) -> Result<Vec<Decimal>, Box<dyn Error>> {
    Ok(load_yahoo_daily_closes_with_dates(path)?
        .into_iter()
        .map(|daily_close| daily_close.closing_price)
        .collect())
}

pub fn load_yahoo_daily_closes_with_dates(
    path: &str,
) -> Result<Vec<YahooDailyClose>, Box<dyn Error>> {
    let file_contents = std::fs::read_to_string(path)?;
    let parsed: YahooChartFile = serde_json::from_str(&file_contents)?;
    let result = parsed
        .chart
        .result
        .first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Yahoo result is empty"))?;
    let closing_prices = &result
        .indicators
        .quote
        .first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Yahoo quote is empty"))?
        .close;

    let mut daily_closes = Vec::new();
    for (timestamp, closing_price) in result.timestamp.iter().zip(closing_prices) {
        let Some(closing_price) = closing_price else {
            continue;
        };
        let date = DateTime::<Utc>::from_timestamp(*timestamp, 0)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid Yahoo timestamp"))?
            .date_naive()
            .to_string();

        daily_closes.push(YahooDailyClose {
            date,
            closing_price: Decimal::try_from(*closing_price)?,
        });
    }

    Ok(daily_closes)
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
