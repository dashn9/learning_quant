use rust_decimal::{Decimal, MathematicalOps};

#[derive(Debug, PartialEq)]
pub struct BasicSummary {
    pub mean: Decimal,
    pub sample_variance: Decimal,
    pub standard_deviation: Decimal,
}

#[derive(Debug, PartialEq)]
pub struct DispersionSummary {
    pub lower_quartile: Decimal,
    pub median: Decimal,
    pub upper_quartile: Decimal,
    pub interquartile_range: Decimal,
    pub standard_deviation: Decimal,
}

pub fn mean(values: &[Decimal]) -> Decimal {
    assert!(!values.is_empty(), "mean requires at least one value");
    values.iter().sum::<Decimal>() / Decimal::from(values.len())
}

pub fn trimmed_mean(values: &[Decimal], trim_percent: usize) -> Decimal {
    assert!(trim_percent < 50, "trim percentage must be below 50");

    let sorted_values = sorted(values);
    let trim_count = sorted_values.len() * trim_percent / 100;
    mean(&sorted_values[trim_count..sorted_values.len() - trim_count])
}

pub fn median(values: &[Decimal]) -> Decimal {
    median_of_sorted(&sorted(values))
}

pub fn sample_variance(values: &[Decimal]) -> Decimal {
    assert!(
        values.len() >= 2,
        "sample variance requires at least two values"
    );

    let average = mean(values);
    let squared_deviations = values
        .iter()
        .map(|value| {
            let deviation = value - average;
            deviation * deviation
        })
        .sum::<Decimal>();

    // N - 1 applies Bessel's correction because this is sample variance.
    squared_deviations / Decimal::from(values.len() - 1)
}

pub fn standard_deviation(values: &[Decimal]) -> Decimal {
    sample_variance(values)
        .sqrt()
        .expect("variance cannot be negative")
}

pub fn basic_summary(values: &[Decimal]) -> BasicSummary {
    let sample_variance = sample_variance(values);
    BasicSummary {
        mean: mean(values),
        sample_variance,
        standard_deviation: sample_variance.sqrt().expect("variance cannot be negative"),
    }
}

pub fn dispersion_summary(values: &[Decimal]) -> DispersionSummary {
    let sorted_values = sorted(values);
    let lower_quartile = sorted_values[sorted_values.len().div_ceil(4) - 1];
    let upper_quartile = sorted_values[(3 * sorted_values.len()).div_ceil(4) - 1];

    DispersionSummary {
        lower_quartile,
        median: median_of_sorted(&sorted_values),
        upper_quartile,
        interquartile_range: upper_quartile - lower_quartile,
        standard_deviation: standard_deviation(values),
    }
}

fn sorted(values: &[Decimal]) -> Vec<Decimal> {
    assert!(!values.is_empty(), "statistics require at least one value");
    let mut sorted_values = values.to_vec();
    sorted_values.sort();
    sorted_values
}

fn median_of_sorted(sorted_values: &[Decimal]) -> Decimal {
    let middle = sorted_values.len() / 2;
    if sorted_values.len().is_multiple_of(2) {
        (sorted_values[middle - 1] + sorted_values[middle]) / Decimal::from(2)
    } else {
        sorted_values[middle]
    }
}
