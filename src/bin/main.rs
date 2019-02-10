use std::env;
use std::process;

use rustventofcode;
use common::Config;
use aoc_day1;
use aoc_day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args_ref: Vec<&str> = Vec::new();
    
    //put the args in an iterator
    for arg in args.iter(){
        args_ref.push(&arg);
    }

    let config = Config::new(&args_ref).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    match config.day{
        1 => aoc_day1::run(config).unwrap(),
        2 => aoc_day2::run(config).unwrap(),
        _ => print!("day not yet supported"),
    }
    
}

