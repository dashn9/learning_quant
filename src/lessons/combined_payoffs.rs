use rust_decimal::{Decimal, dec};

use super::LessonResult;
use crate::statistics;

pub fn run() -> LessonResult {
    let position_y_payoffs = [dec!(10), dec!(30)];
    let position_x_multiplier = dec!(2);
    let position_x_payoffs = position_y_payoffs.map(|payoff| payoff * position_x_multiplier);

    // Each row is an equally likely joint outcome: (X payoff, Y payoff).
    let independent_outcomes = [
        (position_x_payoffs[0], position_y_payoffs[0]),
        (position_x_payoffs[0], position_y_payoffs[1]),
        (position_x_payoffs[1], position_y_payoffs[0]),
        (position_x_payoffs[1], position_y_payoffs[1]),
    ];
    let dependent_outcomes = [
        (position_x_payoffs[0], position_y_payoffs[0]),
        (position_x_payoffs[1], position_y_payoffs[1]),
    ];

    println!("Payoffs are dollars; variance is dollars squared.");
    print_summary("Independent: all four pairings", &independent_outcomes);
    print_summary(
        &format!("Dependent: X = {position_x_multiplier} * Y"),
        &dependent_outcomes,
    );
    println!("\nTry changing position_x_multiplier to -2 and compare X + Y with X - Y.");
    Ok(())
}

fn print_summary(label: &str, joint_outcomes: &[(Decimal, Decimal)]) {
    let position_x_payoffs: Vec<_> = joint_outcomes.iter().map(|&(payoff, _)| payoff).collect();
    let position_y_payoffs: Vec<_> = joint_outcomes.iter().map(|&(_, payoff)| payoff).collect();
    let combined_payoffs: Vec<_> = joint_outcomes
        .iter()
        .map(|&(position_x, position_y)| position_x + position_y)
        .collect();
    let difference_payoffs: Vec<_> = joint_outcomes
        .iter()
        .map(|&(position_x, position_y)| position_x - position_y)
        .collect();

    println!("\n{label}");
    for (position, payoffs) in [
        ("X", position_x_payoffs.as_slice()),
        ("Y", position_y_payoffs.as_slice()),
        ("X + Y", combined_payoffs.as_slice()),
        ("X - Y", difference_payoffs.as_slice()),
    ] {
        println!(
            "  {position:<5}  expected payoff: {:>6}  variance: {:>6}",
            statistics::mean(payoffs),
            payoff_variance(payoffs),
        );
    }
}

fn payoff_variance(payoffs: &[Decimal]) -> Decimal {
    let expected_payoff = statistics::mean(payoffs);
    let squared_deviations = payoffs
        .iter()
        .map(|payoff| {
            let deviation_from_expected_payoff = payoff - expected_payoff;
            deviation_from_expected_payoff * deviation_from_expected_payoff
        })
        .sum::<Decimal>();

    // These are all equally likely outcomes, so use N rather than the sample correction N - 1.
    squared_deviations / Decimal::from(payoffs.len())
}
