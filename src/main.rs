use rust_decimal::{Decimal, dec};
use serde::Deserialize;

use crate::{
    mean::{mean, trimmed_mean},
    median::median,
    std_dev::std_dev,
    variance::variance,
};

mod mean;
mod median;
mod std_dev;
mod utils;
mod variance;

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
fn main() {
    quantifying_trimmed_mean_median_against_mean();
}
