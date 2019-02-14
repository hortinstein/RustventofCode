use common::Config;
use std::collections::HashMap;
//use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

type Fabric = HashMap<(u32, u32), u32>;

struct Patch {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn parse_input(input: &str) -> Patch {
    let mut split = input.split(" ");
    
    let id = (&split.next().unwrap()[1..]).parse().unwrap();
    &split.next().unwrap();
    let x_y = &(&split.next().unwrap().split(",")).collect();
    
    
    dbg!(&(&split.next().unwrap().split(",")));
    Patch {
        id:id,
        x:0,
        y:0,
        width:0,
        height:0,
    }
}

fn part1(input: &str) -> Result<i32> {

    let fabric = Fabric::new();
    for line in input.lines(){
        let patch = parse_input(line);
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
