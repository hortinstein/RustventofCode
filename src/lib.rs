use std::error::Error;

use common::Config;
use aoc_day1;
use aoc_day2;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.day{
        1 => aoc_day1::run(config).unwrap(),
        2 => aoc_day2::run(config).unwrap(),
        _ => print!("day not yet supported"),
    }

    Ok(())
}
