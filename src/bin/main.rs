use std::env;
use std::process;

use rustventofcode;
use common::Config;

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
   
    rustventofcode::run(config).unwrap_or_else(|err| {
        println!("error running advent: {}", err);
        process::exit(1);
    });
}

