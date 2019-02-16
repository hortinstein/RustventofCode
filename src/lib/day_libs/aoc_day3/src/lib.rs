use common::Config;
use std::collections::HashMap;
use regex::Regex;
//use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

type Fabric = HashMap<(u32, u32), u32>;

#[derive(Debug)]
struct Patch {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn parse_input(input: &str) -> Patch {
    let RE: Regex = Regex::new( r"(?x)
                                \#
                                (?P<id>[0-9]+)
                                \s+@\s+
                                (?P<x>[0-9]+),(?P<y>[0-9]+):
                                \s+
                                (?P<width>[0-9]+)x(?P<height>[0-9]+)
                                ").unwrap();
    

        let caps = match RE.captures(input) {
            None => panic!("cannot tell format"),
            Some(caps) => caps,
        };
        Patch {
            id: caps["id"].parse().unwrap(),
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            width: caps["width"].parse().unwrap(),
            height: caps["height"].parse().unwrap(),
        }
}

fn part1(input: &str) -> Result<i32> {

    let fabric = Fabric::new();
    for line in input.lines(){
        let patch = parse_input(line);
        dbg!(patch);
    }

    Ok(1)
}


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
