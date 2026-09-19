use std::error::Error;

mod downside_tail_risk;
mod fat_tails;
mod leverage;
mod mean_reversion;
mod robust_averages;
mod volatility;

pub type LessonResult = Result<(), Box<dyn Error>>;

pub fn run(lesson_name: &str) -> LessonResult {
    match lesson_name {
        "downside-tail-risk" => downside_tail_risk::run(),
        "volatility" => volatility::run(),
        "robust-averages" => robust_averages::run(),
        "fat-tails" => fat_tails::run(),
        "leverage" => leverage::run(),
        "mean-reversion" => mean_reversion::run(),
        _ => Err(format!(
            "unknown lesson '{lesson_name}'. Choose: volatility, robust-averages, fat-tails, leverage, mean-reversion, or downside-tail-risk"
        )
        .into()),
    }
}
