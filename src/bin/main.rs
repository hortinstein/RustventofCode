use std::env;
use std::process;

use common::Config;
use rustventofcode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args_ref: Vec<&str> = Vec::new();

    args_ref.push("progname");
    args_ref.push(&args[1]);

    let mut input_str = String::from("src/lib/day_libs/aoc_day");
    input_str.push_str(&args[1]);
    input_str.push_str("/input/input.txt");

    args_ref.push(&input_str);
    let config = Config::new(&args_ref).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    rustventofcode::run(config).unwrap_or_else(|err| {
        println!("error running advent: {}", err);
        process::exit(1);
    });
}
