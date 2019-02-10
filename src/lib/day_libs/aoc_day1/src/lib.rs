
use common::Config;
use std::error;
use std::fmt;
use std::num::ParseIntError;


pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(3 + 2, 4);
    }
}
