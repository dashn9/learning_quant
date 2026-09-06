use rust_decimal::{Decimal, MathematicalOps, dec};

use crate::mean::mean;

pub fn variance(efx: &[Decimal]) -> Decimal {
    let mean = mean(efx);
    let mut xminusmeansq: Decimal = Decimal::default();
    for x in efx {
        xminusmeansq += (x - mean).powd(dec!(2));
    }
    // Traditionally, it should be N, but N - 1 was used(Bessel's correction)
    xminusmeansq / Decimal::from(efx.len() - 1)
}