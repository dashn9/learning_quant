use rust_decimal::{Decimal, MathematicalOps};

use crate::variance::variance;

pub fn std_dev(efx: &[Decimal]) -> Decimal {
    // Variance is a sum of squares, so it is never negative and sqrt never returns None
    variance(efx).sqrt().unwrap()
}
