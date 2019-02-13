use common::Config;
use std::collections::HashMap;
//use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn part1(input: &str) -> Result<i32> {
    Ok(check_sum)
}

use difference::{Changeset, Difference};

fn part2(input: &str) -> Result<i32> {
    Ok(1)
}

pub fn run(config: Config) -> Result<()> {
    println!("Day3:");
    part1(&(config.contents))?;
    part2(&(config.contents))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(4, 2 + 2);
    }
    #[test]
    fn part2() {
        assert_eq!(4, 2 + 2);
    }

}
