
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub contents: String 
}

impl Config {
    pub fn new(args:&std::vec::Vec<&str>) -> Result<Config, &'static str> {
        let mut iter = args.iter();
        let _ = match iter.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a program name"),
        };
        
        let filename = match iter.next() {
            Some(arg) => arg.to_string(),
            None => return Err("Didn't get a file name"),
        };

        let contents = match fs::read_to_string(&filename){
            Ok(contents) => contents,
            Err(err) => {
                dbg!(filename);
                return Err("Didn't get file contents for file");
            }
        };

        Ok(Config { filename, contents })
}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_file_open() {
        let args = vec!("test",".gitignore");
        dbg!(&args);
        let config = Config::new(&args).unwrap();
        assert!(config.contents.contains("targets"));
    }
}
