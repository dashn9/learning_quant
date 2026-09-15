use rust_decimal::{Decimal};

pub fn mean(efx: &[Decimal]) -> Decimal {
    efx.iter().sum::<Decimal>() / Decimal::from(efx.len())
}

pub fn trimmed_mean(efx: &[Decimal], trim: usize) -> Decimal {
    let mut s = efx.to_vec();
    s.sort();
    // floored by default
    let k = s.len() * trim / 100;
    mean(&s[k..s.len() - k])
}