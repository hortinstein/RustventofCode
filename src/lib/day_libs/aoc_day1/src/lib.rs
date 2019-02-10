
use common::Config;
use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn part1(input: &str) -> Result<i32> {
    let mut num:i32 = 0;
    for line in input.lines(){
        let change: i32 = line.parse()?;
        num+=change;
    }
    println!("Part1: {}",num);
    Ok(num)

}

fn part2(input: &str) -> Result<i32> {
    let mut freq:i32 = 0;
    let mut seen = HashSet::new();
    seen.insert(freq);
    loop{
        print!("loop ");    
        for line in input.lines(){
            let change: i32 = line.parse()?;
            freq+=change;
            if seen.contains(&freq){
                println!("Part2: {}",freq);
                return Ok(freq);
            }
            seen.insert(freq);
        }
    }
    Err(From::from("never found a repeat"))
}


pub fn run(config: Config) -> Result<()> {
    println!("Day1:");
    part1(&(config.contents));
    part2(&(config.contents)); 
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = "-1\n0\n2\n3"; //4
        let result = super::part1(input).unwrap();
        dbg!(result);
        assert_eq!(4,result);
    }
    #[test]
    fn part2() {
        let input = "-1\n0\n1\n3"; //4
        dbg!(input);
        let result = super::part2(input).unwrap();
        assert_eq!(-1,result);
    }

}
