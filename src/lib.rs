use std::error::Error;

use aoc_day1;
use aoc_day2;
use aoc_day3;
use common::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.day {
        1 => aoc_day1::run(config).unwrap(),
        2 => aoc_day2::run(config).unwrap(),
        3 => aoc_day2::run(config).unwrap(),
        _ => print!("day not yet supported"),
    }

    Ok(())
}
