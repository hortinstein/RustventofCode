
use common::Config;
use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn part1(input: &str) -> Result<i32> {
    let (mut twos, mut threes) = (0, 0);
    for line in input.lines(){
        //creates a hashmap with ascii a to z  
        let mut char_count = HashMap::new();
        char_count.extend((b'a'..=b'z').map(|c| (c as char,0)));
        //iterate through all of the letters
        for b in line.chars(){
            //increment the char count
            *char_count.get_mut(&b).unwrap() = char_count[&b] + 1;
        }
        //check to see how many's count is 2
        if char_count.iter().any(|(&c,n)| *n == 2) {
            twos += 1;
        }
        //check to see how many's count is 3
        if char_count.iter().any(|(&c,n)| *n == 3) {
            threes += 1;
        }
    }
    let check_sum = twos * threes;
    dbg!(check_sum);
    Ok(1)

}

fn part2(input: &str) -> Result<i32> {
    
    
    Ok(1)
}


pub fn run(config: Config) -> Result<()> {
    println!("Day2:");
    part1(&(config.contents))?;
    part2(&(config.contents))?; 
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(4,2+2);
    }
    #[test]
    fn part2() {
        assert_eq!(4,2+2);
    }

}
