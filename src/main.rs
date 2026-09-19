mod data_sources;
mod lessons;
mod output;
mod statistics;

fn main() -> lessons::LessonResult {
    let lesson_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "leverage".to_owned());

    lessons::run(&lesson_name)
}
