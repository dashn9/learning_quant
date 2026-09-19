use rust_decimal::{Decimal, dec};
use serde::Deserialize;

use super::LessonResult;
use crate::{output, statistics};

#[derive(Deserialize)]
struct DailyPrice {
    date: String,
    #[serde(rename = "open")]
    opening_price: Decimal,
    #[serde(rename = "close")]
    closing_price: Decimal,
}

pub fn run() -> LessonResult {
    let mut reader = csv::Reader::from_path("assets/spy-aug-2026")?;
    let mut percentage_changes = Vec::new();

    for daily_price in reader.deserialize() {
        let daily_price: DailyPrice = daily_price?;
        let percentage_change = (daily_price.closing_price - daily_price.opening_price) * dec!(100)
            / daily_price.opening_price;

        output::print_price_change(
            &daily_price.date,
            daily_price.opening_price,
            daily_price.closing_price,
            percentage_change,
        );
        percentage_changes.push(percentage_change);
    }

    output::print_volatility_summary(&statistics::basic_summary(&percentage_changes));
    Ok(())
}
