use rust_decimal::{Decimal, dec};

use super::LessonResult;
use crate::{output, statistics};

/// For Y = aX + b: E[Y] = a * E[X] + b, and Var(Y) = a^2 * Var(X).
/// The shift b changes the mean but cancels out of every deviation.
pub fn run() -> LessonResult {
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

    let shifted_returns = affine_transform(&unleveraged_returns, Decimal::ONE, fixed_return_offset);
    let scaled_returns = affine_transform(&unleveraged_returns, leverage_multiplier, Decimal::ZERO);
    let scaled_and_shifted_returns = affine_transform(
        &unleveraged_returns,
        leverage_multiplier,
        fixed_return_offset,
    );

    for (label, values) in [
        (
            "X                 (unleveraged)",
            unleveraged_returns.as_slice(),
        ),
        ("X + 0.01          (shift only)", shifted_returns.as_slice()),
        ("2.5X              (scale only)", scaled_returns.as_slice()),
        (
            "Y = 2.5X + 0.01   (both)",
            scaled_and_shifted_returns.as_slice(),
        ),
    ] {
        output::print_leverage_effect(label, values, &statistics::basic_summary(values));
    }

    let baseline_variance = statistics::sample_variance(&unleveraged_returns);
    output::print_variance_ratios(
        statistics::sample_variance(&scaled_and_shifted_returns) / baseline_variance,
        leverage_multiplier * leverage_multiplier,
        statistics::sample_variance(&shifted_returns) / baseline_variance,
    );
    Ok(())
}

fn affine_transform(values: &[Decimal], multiplier: Decimal, offset: Decimal) -> Vec<Decimal> {
    values
        .iter()
        .map(|value| value * multiplier + offset)
        .collect()
}
