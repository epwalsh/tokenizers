use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;

mod tokenizer;

pub use crate::tokenizer::Tokenizer;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // First arg is the path of the binary.
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { filename })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let tokenizer = Tokenizer::english();
    let file = File::open(&config.filename)?;
    let file = BufReader::new(file);
    for line in file.lines() {
        let line = line?;
        let tokens = tokenizer.tokenize(line.as_str());
        println!("{:?}", tokens);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_ok() {
        let config = Config { filename: String::from("tests/fixtures/poem.txt") };
        let result = run(&config);
        if let Err(_) = result {
            panic!("Result of 'run' should be Ok");
        };
    }

    #[test]
    fn test_run_err() {
        let config = Config { filename: String::from("non_existant_file.txt") };
        let result = run(&config);
        if let Ok(_) = result {
            panic!("Result of 'run' should be Err");
        };
    }
}
