use rust_decimal::{Decimal, dec};
use serde::Deserialize;

use crate::{
    mean::{mean, trimmed_mean},
    median::median,
    range::{
        get_interquartile_range, interquartile_get_lower_quartile, interquartile_get_upper_quartile,
    },
    std_dev::std_dev,
    variance::variance,
};

mod data_sources;
mod mean;
mod median;
mod std_dev;
mod utils;
mod variance;
mod range;

#[derive(Debug, Deserialize)]
struct SpyAug2026 {
    date: String,
    #[serde(rename = "open")]
    opening_price: Decimal,
    #[serde(rename = "close")]
    closing_price: Decimal,
}

fn quantifying_volatility_ch_1_3() {
    let mut tmp_data = csv::Reader::from_path("assets/spy-aug-2026").unwrap();
    let mut spy_data: Vec<SpyAug2026> = Vec::new();
    let mut percentage_change: Vec<Decimal> = Vec::new();
    for d in tmp_data.deserialize() {
        let spy_unit: SpyAug2026 = d.unwrap();
        let change = (spy_unit.closing_price - spy_unit.opening_price) * dec!(100.0)
            / spy_unit.opening_price;
        println!(
            "{}  open: {:>8}  close: {:>8}  change: {:>7.3}%",
            spy_unit.date, spy_unit.opening_price, spy_unit.closing_price, change
        );
        percentage_change.push(change);
        spy_data.push(spy_unit);
    }
    let change_mean = mean(&percentage_change);
    println!("mean: {}", change_mean);
    println!("variance: {}", variance(&percentage_change));
    println!("std dev: {}", std_dev(&percentage_change));
}

#[derive(Debug, Deserialize)]
struct SP500Crash2020 {
    date: String,
    close: Decimal,
}

//Understand why median and trimmed mean reflect typical returns better during stress.
fn quantifying_trimmed_mean_median_against_mean() {
    let mut tmp_data = csv::Reader::from_path("assets/sp500-crash-2020").unwrap();
    let mut closing = Vec::new();
    let mut prev_closing = Decimal::from(0);
    for (i, data) in tmp_data.deserialize().into_iter().enumerate() {
        let data: SP500Crash2020 = data.unwrap();
        println!("{:>3}  {}  close: {:>8}", i, data.date, data.close);
        if i == 0 {
            // Do nothing here 0 or the actual value skew the averages.
        } else {
            let point_change = data.close - prev_closing;
            println!("     change: {:>8}", point_change);
            closing.push(point_change);
        }
        prev_closing = data.close
    }

    let mean = mean(&closing);
    let trimmed_mean = trimmed_mean(&closing, 5);
    let median = median(&closing);
    println!("mean: {}", mean);
    println!("trimmed mean: {}", trimmed_mean);
    println!("median: {}", median);
}

// Identify non-Gaussian fat tails and downside asymmetric risk in alternative assets.
fn quantifying_fat_tails_crypto_vs_treasury() {
    let btc_closes = data_sources::load_yahoo_daily_closes("assets/btc-usd-2017-2023");
    let treasury_yields = data_sources::load_fred_daily_values("assets/DGS10-2015-2023.csv");

    // Prices become percent returns, rates become absolute changes in percentage points.
    let btc_returns: Vec<Decimal> = btc_closes
        .windows(2)
        .map(|pair| (pair[1] - pair[0]) * dec!(100) / pair[0])
        .collect();
    let treasury_changes: Vec<Decimal> = treasury_yields
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();

    print_sample_block("first 10", &btc_returns, &treasury_changes, 0, 0);
    print_sample_block(
        "mid 10",
        &btc_returns,
        &treasury_changes,
        btc_returns.len() / 2,
        treasury_changes.len() / 2,
    );
    print_sample_block(
        "last 10",
        &btc_returns,
        &treasury_changes,
        btc_returns.len() - SAMPLE_SIZE,
        treasury_changes.len() - SAMPLE_SIZE,
    );

    report_dispersion("BTC daily returns (%)", &btc_returns);
    report_dispersion("DGS10 daily changes (pp)", &treasury_changes);
}

/// A Gaussian sits at IQR/StdDev = 1.349. Lower means fatter tails, because
/// outliers inflate StdDev while leaving the middle 50% untouched.
fn report_dispersion(label: &str, series: &[Decimal]) {
    let first_quartile = interquartile_get_lower_quartile(series);
    let third_quartile = interquartile_get_upper_quartile(series);
    let series_median = median(series);
    let interquartile_range = get_interquartile_range(series);
    let standard_deviation = std_dev(series);

    println!("\n{}  (n = {})", label, series.len());
    println!("  Q1             {:>10.4}", first_quartile);
    println!("  median         {:>10.4}", series_median);
    println!("  Q3             {:>10.4}", third_quartile);
    println!("  IQR            {:>10.4}", interquartile_range);
    println!("  std dev        {:>10.4}", standard_deviation);
    println!(
        "  IQR / std dev  {:>10.4}   (Gaussian = 1.3490)",
        interquartile_range / standard_deviation
    );
    println!("  median - Q1    {:>10.4}", series_median - first_quartile);
    println!("  Q3 - median    {:>10.4}", third_quartile - series_median);
}

const SAMPLE_SIZE: usize = 10;

/// Each column is sampled from its own series, so a row shows two values that
/// are not the same day. Read down a column, never across.
fn print_sample_block(
    title: &str,
    left: &[Decimal],
    right: &[Decimal],
    left_start: usize,
    right_start: usize,
) {
    println!("\n{}", title);
    println!("{:<16}{:>16}{:>20}", "row", "BTC returns (%)", "DGS10 changes (pp)");
    for offset in 0..SAMPLE_SIZE {
        println!(
            "{:<16}{:>16.4}{:>20.4}",
            offset,
            left[left_start + offset],
            right[right_start + offset],
        );
    }
}

/// For Y = aX + b:  E[Y] = a * E[X] + b,  and  Var(Y) = a^2 * Var(X).
/// The shift b lands on the mean but cancels out of every deviation, so variance ignores it.
fn leverage_portfolio_management() {
    // Decimal fractions, not percent: 0.0228 is a 2.28% day.
    let unleveraged_returns = [
        dec!(0.0228),
        dec!(0.0212),
        dec!(0.0233),
        dec!(0.0579),
        dec!(0.0122),
        dec!(0.0189),
    ];
    let leverage_multiplier = dec!(2.5);
    let fixed_return_offset = dec!(0.01);

    let scaled_and_shifted_returns: Vec<Decimal> = unleveraged_returns
        .iter()
        .copied()
        .map(|daily_return| daily_return * leverage_multiplier + fixed_return_offset)
        .collect();
    let scaled_returns: Vec<Decimal> = unleveraged_returns
        .iter()
        .copied()
        .map(|daily_return| daily_return * leverage_multiplier)
        .collect();
    let shifted_returns: Vec<Decimal> = unleveraged_returns
        .iter()
        .copied()
        .map(|daily_return| daily_return + fixed_return_offset)
        .collect();

    report_leverage_effect("X                 (unleveraged)", &unleveraged_returns);
    report_leverage_effect("X + 0.01          (shift only)", &shifted_returns);
    report_leverage_effect("2.5X              (scale only)", &scaled_returns);
    report_leverage_effect("Y = 2.5X + 0.01   (both)", &scaled_and_shifted_returns);

    let baseline_variance = variance(&unleveraged_returns);
    println!(
        "\nvariance ratio  Y / X            {:.6}   predicted a^2 = {}",
        variance(&scaled_and_shifted_returns) / baseline_variance,
        leverage_multiplier * leverage_multiplier
    );
    println!(
        "variance ratio  shift-only / X   {:.6}   predicted 1, a shift changes no distance",
        variance(&shifted_returns) / baseline_variance
    );
}

/// Prints a series beside its mean and variance so the four variants compare row by row.
fn report_leverage_effect(label: &str, series: &[Decimal]) {
    print!("\n{:<34}", label);
    for value in series {
        print!("{:>10.4}", value);
    }
    println!();
    println!(
        "  mean {:>12.6}   variance {:>14.8}   std dev {:>10.6}",
        mean(series),
        variance(series),
        std_dev(series)
    );
}

fn main() {
    leverage_portfolio_management();
}
