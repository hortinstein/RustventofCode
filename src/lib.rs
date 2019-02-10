use std::error::Error;
use std::fs;

use common::Config;
use aoc_day1;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    println!("With text:\n{}", config.contents);

    Ok(())
}
