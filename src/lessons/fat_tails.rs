use rust_decimal::{Decimal, dec};

use super::LessonResult;
use crate::{data_sources, output, statistics};

const SAMPLE_SIZE: usize = 10;

// Identify non-Gaussian fat tails and downside asymmetric risk in alternative assets.
pub fn run() -> LessonResult {
    let bitcoin_closes = data_sources::load_yahoo_daily_closes("assets/btc-usd-2017-2023")?;
    let treasury_yields = data_sources::load_fred_daily_values("assets/DGS10-2015-2023.csv")?;

    // Prices become percent returns, rates become absolute changes in percentage points.
    let bitcoin_returns: Vec<Decimal> = bitcoin_closes
        .windows(2)
        .map(|prices| (prices[1] - prices[0]) * dec!(100) / prices[0])
        .collect();
    let treasury_changes: Vec<Decimal> = treasury_yields
        .windows(2)
        .map(|yields| yields[1] - yields[0])
        .collect();

    for (title, bitcoin_start, treasury_start) in [
        ("first 10", 0, 0),
        (
            "mid 10",
            bitcoin_returns.len() / 2,
            treasury_changes.len() / 2,
        ),
        (
            "last 10",
            bitcoin_returns.len().saturating_sub(SAMPLE_SIZE),
            treasury_changes.len().saturating_sub(SAMPLE_SIZE),
        ),
    ] {
        output::print_sample_block(
            title,
            &bitcoin_returns,
            &treasury_changes,
            bitcoin_start,
            treasury_start,
            SAMPLE_SIZE,
        );
    }

    output::print_dispersion(
        "BTC daily returns (%)",
        bitcoin_returns.len(),
        &statistics::dispersion_summary(&bitcoin_returns),
    );
    output::print_dispersion(
        "DGS10 daily changes (pp)",
        treasury_changes.len(),
        &statistics::dispersion_summary(&treasury_changes),
    );
    Ok(())
}
