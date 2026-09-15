use rust_decimal::Decimal;


pub fn interquartile_get_upper_quartile(data: &[Decimal]) -> Decimal {
    let mut data = data.to_vec();
    data.sort();
    let upper_quartile = (3 * data.len()).div_ceil(4);
    data[upper_quartile - 1]
}

pub fn interquartile_get_lower_quartile(data: &[Decimal]) -> Decimal {
    let mut data = data.to_vec();
    data.sort();
    let lower_quartile = data.len().div_ceil(4);
    data[lower_quartile - 1]
}

pub fn get_interquartile_range(data: &[Decimal]) -> Decimal {
    interquartile_get_upper_quartile(data) - interquartile_get_lower_quartile(data)
}