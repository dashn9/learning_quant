# My Public Quant Learning Journey

Run `cargo run -- combined-payoffs` to compare independent payoffs with X = 2Y.
Both cases keep the same individual payoff distributions and show the expectations
and variances for X, Y, X + Y, and X - Y.

In `src/lessons/combined_payoffs.rs`, change `position_x_multiplier` to `dec!(-2)`
and predict which combined position will have greater variance before running it again.
