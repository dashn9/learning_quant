use rust_decimal::{Decimal};

pub fn mean(efx: &[Decimal]) -> Decimal {
    efx.iter().sum::<Decimal>() / Decimal::from(efx.len())
}