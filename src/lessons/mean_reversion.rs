use rust_decimal::{Decimal, dec};

use super::LessonResult;
use crate::{data_sources, output, statistics};

const LOOKBACK_DAYS: usize = 20;
const ENTRY_THRESHOLD: Decimal = dec!(2);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EntrySignal {
    Long,
    Short,
    NoEntry,
}

impl EntrySignal {
    fn label(&self) -> &'static str {
        match self {
            Self::Long => "LONG",
            Self::Short => "SHORT",
            Self::NoEntry => "-",
        }
    }
}

pub fn run() -> LessonResult {
    mean_reversion_trading_sma()
}

// Build fundamental quantitative mean-reversion trading signals.
fn mean_reversion_trading_sma() -> LessonResult {
    let daily_closes = data_sources::load_yahoo_daily_closes("assets/spy-2017")?;
    if daily_closes.len() <= LOOKBACK_DAYS + 1 {
        return Err("mean reversion requires at least 22 closing prices".into());
    }

    let mut long_entries = 0;
    let mut short_entries = 0;
    let mut evaluated_days = 0;
    let mut winning_trades = 0;
    let mut losing_trades = 0;
    let mut flat_trades = 0;
    let mut total_price_difference = Decimal::ZERO;
    output::print_mean_reversion_header(LOOKBACK_DAYS, ENTRY_THRESHOLD);

    for current_day_index in LOOKBACK_DAYS..daily_closes.len() - 1 {
        let baseline_prices = &daily_closes[current_day_index - LOOKBACK_DAYS..current_day_index];
        let current_price = daily_closes[current_day_index];
        let next_closing_price = daily_closes[current_day_index + 1];
        let moving_average = statistics::mean(baseline_prices);
        let price_standard_deviation = statistics::standard_deviation(baseline_prices);

        if price_standard_deviation == Decimal::ZERO {
            continue;
        }

        let z_score = (current_price - moving_average) / price_standard_deviation;
        let entry_signal = classify_entry_signal(z_score);
        let price_difference =
            calculate_price_difference(entry_signal, current_price, next_closing_price);
        match entry_signal {
            EntrySignal::Long => long_entries += 1,
            EntrySignal::Short => short_entries += 1,
            EntrySignal::NoEntry => {}
        }
        if let Some(price_difference) = price_difference {
            total_price_difference += price_difference;
            match price_difference.cmp(&Decimal::ZERO) {
                std::cmp::Ordering::Greater => winning_trades += 1,
                std::cmp::Ordering::Less => losing_trades += 1,
                std::cmp::Ordering::Equal => flat_trades += 1,
            }
        }

        evaluated_days += 1;
        output::print_mean_reversion_observation(output::MeanReversionRow {
            day: current_day_index + 1,
            current_price,
            next_closing_price,
            moving_average,
            standard_deviation: price_standard_deviation,
            z_score,
            signal: entry_signal.label(),
            price_difference,
        });
    }

    output::print_mean_reversion_summary(
        evaluated_days,
        long_entries,
        short_entries,
        winning_trades,
        losing_trades,
        flat_trades,
        total_price_difference,
    );
    Ok(())
}

fn classify_entry_signal(z_score: Decimal) -> EntrySignal {
    if z_score > ENTRY_THRESHOLD {
        EntrySignal::Short
    } else if z_score < -ENTRY_THRESHOLD {
        EntrySignal::Long
    } else {
        EntrySignal::NoEntry
    }
}

fn calculate_price_difference(
    entry_signal: EntrySignal,
    entry_price: Decimal,
    next_closing_price: Decimal,
) -> Option<Decimal> {
    match entry_signal {
        EntrySignal::Long => Some(next_closing_price - entry_price),
        EntrySignal::Short => Some(entry_price - next_closing_price),
        EntrySignal::NoEntry => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_mean_reversion_entries() {
        assert_eq!(classify_entry_signal(dec!(2.1)), EntrySignal::Short);
        assert_eq!(classify_entry_signal(dec!(-2.1)), EntrySignal::Long);
        assert_eq!(classify_entry_signal(dec!(2)), EntrySignal::NoEntry);
        assert_eq!(classify_entry_signal(dec!(-2)), EntrySignal::NoEntry);
    }

    #[test]
    fn calculates_one_share_price_differences() {
        assert_eq!(
            calculate_price_difference(EntrySignal::Long, dec!(100), dec!(98)),
            Some(dec!(-2))
        );
        assert_eq!(
            calculate_price_difference(EntrySignal::Short, dec!(100), dec!(98)),
            Some(dec!(2))
        );
        assert_eq!(
            calculate_price_difference(EntrySignal::NoEntry, dec!(100), dec!(98)),
            None
        );
    }
}
