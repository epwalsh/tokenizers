use std::error::Error;
use std::fs;
use std::str;

mod tokenizer;

pub use crate::tokenizer::Tokenizer;

#[derive(Debug, PartialEq)]
pub struct Config {
    filename: String,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tokenizer = Tokenizer::english();
    let contents = fs::read_to_string(config.filename)?;
    for line in contents.lines() {
        let tokens = tokenizer.tokenize(line);
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
        let result = run(config);
        if let Err(_) = result {
            panic!("Result of 'run' should be Ok");
        };
    }

    #[test]
    fn test_run_err() {
        let config = Config { filename: String::from("non_existant_file.txt") };
        let result = run(config);
        if let Ok(_) = result {
            panic!("Result of 'run' should be Err");
        };
    }
}
