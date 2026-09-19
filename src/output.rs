use rust_decimal::Decimal;

use crate::statistics::{BasicSummary, DispersionSummary};

pub fn print_price_change(
    date: &str,
    opening_price: Decimal,
    closing_price: Decimal,
    percentage_change: Decimal,
) {
    println!(
        "{date}  open: {opening_price:>8}  close: {closing_price:>8}  change: {percentage_change:>7.3}%"
    );
}

pub fn print_volatility_summary(summary: &BasicSummary) {
    println!("mean: {}", summary.mean);
    println!("variance: {}", summary.sample_variance);
    println!("std dev: {}", summary.standard_deviation);
}

pub fn print_closing_price(
    row: usize,
    date: &str,
    closing_price: Decimal,
    point_change: Option<Decimal>,
) {
    println!("{row:>3}  {date}  close: {closing_price:>8}");
    if let Some(point_change) = point_change {
        println!("     change: {point_change:>8}");
    }
}

pub fn print_average_comparison(mean: Decimal, trimmed_mean: Decimal, median: Decimal) {
    println!("mean: {mean}");
    println!("trimmed mean: {trimmed_mean}");
    println!("median: {median}");
}

/// Each column is sampled from its own series, so paired values are not from
/// the same day. Read down a column, never across a row.
pub fn print_sample_block(
    title: &str,
    left: &[Decimal],
    right: &[Decimal],
    left_start: usize,
    right_start: usize,
    sample_size: usize,
) {
    println!("\n{title}");
    println!(
        "{:<16}{:>16}{:>20}",
        "row", "BTC returns (%)", "DGS10 changes (pp)"
    );

    for (row, (left_value, right_value)) in left
        .iter()
        .skip(left_start)
        .zip(right.iter().skip(right_start))
        .take(sample_size)
        .enumerate()
    {
        println!("{row:<16}{left_value:>16.4}{right_value:>20.4}");
    }
}

/// A Gaussian has IQR/StdDev = 1.349. Lower values indicate fatter tails:
/// outliers inflate StdDev while leaving the middle 50% relatively unchanged.
pub fn print_dispersion(label: &str, sample_size: usize, summary: &DispersionSummary) {
    println!("\n{label}  (n = {sample_size})");
    println!("  Q1             {:>10.4}", summary.lower_quartile);
    println!("  median         {:>10.4}", summary.median);
    println!("  Q3             {:>10.4}", summary.upper_quartile);
    println!("  IQR            {:>10.4}", summary.interquartile_range);
    println!("  std dev        {:>10.4}", summary.standard_deviation);
    println!(
        "  IQR / std dev  {:>10.4}   (Gaussian = 1.3490)",
        summary.interquartile_range / summary.standard_deviation
    );
    println!(
        "  median - Q1    {:>10.4}",
        summary.median - summary.lower_quartile
    );
    println!(
        "  Q3 - median    {:>10.4}",
        summary.upper_quartile - summary.median
    );
}

/// Prints a series beside its mean and variance so variants compare row by row.
pub fn print_leverage_effect(label: &str, values: &[Decimal], summary: &BasicSummary) {
    print!("\n{label:<34}");
    for value in values {
        print!("{value:>10.4}");
    }
    println!();
    println!(
        "  mean {:>12.6}   variance {:>14.8}   std dev {:>10.6}",
        summary.mean, summary.sample_variance, summary.standard_deviation
    );
}

pub fn print_variance_ratios(
    scaled_and_shifted_ratio: Decimal,
    predicted_scaled_ratio: Decimal,
    shifted_ratio: Decimal,
) {
    println!(
        "\nvariance ratio  Y / X            {scaled_and_shifted_ratio:.6}   predicted a^2 = {predicted_scaled_ratio}"
    );
    println!(
        "variance ratio  shift-only / X   {shifted_ratio:.6}   predicted 1, a shift changes no distance"
    );
}

pub fn print_mean_reversion_header(lookback_days: usize, entry_threshold: Decimal) {
    println!(
        "SPY mean reversion: current close versus previous {lookback_days} closes (|Z| > {entry_threshold})"
    );
    println!(
        "{:>5}{:>13}{:>13}{:>13}{:>13}{:>10}{:>12}{:>13}",
        "day", "entry", "next close", "SMA", "std dev", "Z", "signal", "price diff"
    );
}

pub struct MeanReversionRow<'a> {
    pub day: usize,
    pub current_price: Decimal,
    pub next_closing_price: Decimal,
    pub moving_average: Decimal,
    pub standard_deviation: Decimal,
    pub z_score: Decimal,
    pub signal: &'a str,
    pub price_difference: Option<Decimal>,
}

pub fn print_mean_reversion_observation(row: MeanReversionRow<'_>) {
    let price_difference = row
        .price_difference
        .map(|value| format!("{value:.4}"))
        .unwrap_or_else(|| "-".to_owned());
    println!(
        "{:>5}{:>13.4}{:>13.4}{:>13.4}{:>13.4}{:>10.4}{:>12}{price_difference:>13}",
        row.day,
        row.current_price,
        row.next_closing_price,
        row.moving_average,
        row.standard_deviation,
        row.z_score,
        row.signal,
    );
}

pub fn print_mean_reversion_summary(
    evaluated_days: usize,
    long_entries: usize,
    short_entries: usize,
    winning_trades: usize,
    losing_trades: usize,
    flat_trades: usize,
    total_price_difference: Decimal,
) {
    println!("\nEvaluated days: {evaluated_days}");
    println!("Long entries:   {long_entries}");
    println!("Short entries:  {short_entries}");
    println!("Winning trades: {winning_trades}");
    println!("Losing trades:  {losing_trades}");
    println!("Flat trades:    {flat_trades}");
    println!("Cumulative gross P&L (1 share per trade): {total_price_difference:+.4} USD");
}
