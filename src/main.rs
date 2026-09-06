use rust_decimal::{Decimal, dec};
use serde::Deserialize;

use crate::{mean::mean, std_dev::std_dev, variance::variance};

mod mean;
mod std_dev;
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
        let change = (spy_unit.closing_price - spy_unit.opening_price) * dec!(100.0) / spy_unit.opening_price;
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

fn main() {
    quantifying_volatility_ch_1_3();
}
