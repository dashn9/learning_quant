use rust_decimal::{Decimal, MathematicalOps, dec};

use super::LessonResult;
use crate::data_sources;

struct DailyReturn {
    date: String,
    percentage_return: Decimal,
}

pub fn run() -> LessonResult {
    downside_extreme_tail_risk()
}

fn chebyshev_max_probability(distance_from_mean: Decimal, variance: Decimal) -> Decimal {
    variance / distance_from_mean.powu(2)
}

fn chebyshev_max_probability_beyond_3_5_standard_deviations(
    standard_deviation: Decimal,
    mean: Decimal,
) -> Decimal {
    let signed_downside_deviation = -standard_deviation * dec!(3.5);
    let downside_return_cutoff = mean + signed_downside_deviation;
    let distance_from_mean = mean - downside_return_cutoff;

    // The cutoff is a return value; Chebyshev uses its distance from the mean.
    chebyshev_max_probability(distance_from_mean, standard_deviation.powu(2))
}

fn downside_extreme_tail_risk() -> LessonResult {
    let illustrative_upper_bound =
        chebyshev_max_probability_beyond_3_5_standard_deviations(dec!(4), dec!(25));
    println!(
        "Illustrative Chebyshev upper bound: {:.2}%",
        illustrative_upper_bound * dec!(100)
    );
    Ok(())
}
