use rust_decimal::Decimal;
use serde::Deserialize;

use super::LessonResult;
use crate::{output, statistics};

#[derive(Deserialize)]
struct ClosingPrice {
    date: String,
    close: Decimal,
}

// Understand why median and trimmed mean reflect typical returns better during stress.
pub fn run() -> LessonResult {
    let mut reader = csv::Reader::from_path("assets/sp500-crash-2020")?;
    let mut previous_closing_price = None;
    let mut point_changes = Vec::new();

    for (row, closing_price) in reader.deserialize().enumerate() {
        let closing_price: ClosingPrice = closing_price?;
        let point_change =
            previous_closing_price.map(|previous_price| closing_price.close - previous_price);

        output::print_closing_price(row, &closing_price.date, closing_price.close, point_change);
        // The first row is omitted; zero or its close would skew the averages.
        if let Some(point_change) = point_change {
            point_changes.push(point_change);
        }
        previous_closing_price = Some(closing_price.close);
    }

    output::print_average_comparison(
        statistics::mean(&point_changes),
        statistics::trimmed_mean(&point_changes, 5),
        statistics::median(&point_changes),
    );
    Ok(())
}
