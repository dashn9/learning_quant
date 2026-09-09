use rust_decimal::{Decimal, dec};

use crate::utils::is_even;

pub fn median(data: &[Decimal]) -> Decimal {
    let mut data = data.to_vec();
    data.sort();
    let middle = data.len() / 2;
    if is_even(data.len()) {
        let p1 = data[middle - 1];
        let p2 = data[middle];
        (p1 + p2) / dec!(2)
    } else {
        data[middle]
    }
}