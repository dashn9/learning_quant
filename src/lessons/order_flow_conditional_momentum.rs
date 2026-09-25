use rust_decimal::{Decimal, dec};

use super::LessonResult;

const PROBABILITY_OF_UP_TICK: Decimal = dec!(0.51);
const PROBABILITY_OF_UP_AFTER_UP_TICK: Decimal = dec!(0.64);

pub fn run() -> LessonResult {
    // Assume only the immediately preceding tick affects the next tick.
    let probability_of_three_consecutive_up_ticks =
        PROBABILITY_OF_UP_TICK * PROBABILITY_OF_UP_AFTER_UP_TICK * PROBABILITY_OF_UP_AFTER_UP_TICK;

    println!("Assuming the next tick depends only on the previous tick:");
    println!(
        "P(Up, Up, Up) = {} * {} * {} = {:.4}%",
        PROBABILITY_OF_UP_TICK,
        PROBABILITY_OF_UP_AFTER_UP_TICK,
        PROBABILITY_OF_UP_AFTER_UP_TICK,
        probability_of_three_consecutive_up_ticks * dec!(100),
    );
    Ok(())
}
