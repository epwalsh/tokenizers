use std::error::Error;
use std::fs;

mod tokenizer;

pub use crate::tokenizer::Tokenizer;


#[derive(Debug,PartialEq)]
pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            return Err("too many arguments");
        } else if args.len() < 2 {
            return Err("not enough arguments");
        }
        // TODO: avoid `clone()`-ing
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Reading input file {}", config.filename);

    // TODO: make more efficient. Don't need to read immediately into one big string.
    // Can iterator over lines lazily.
    let contents = fs::read_to_string(config.filename)?;
    let lines = contents.split_terminator("\n");

    // Initialize tokenizer.
    let tokenizer = Tokenizer::english();

    for line in lines {
        let tokens = tokenizer.tokenize(line);
        println!("{:?}", tokens);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let args = vec![String::from("./bin/tokenize"), String::from("foo.txt")];
        let config = Config::new(&args[..]).unwrap();
        assert_eq!(config.filename, "foo.txt");
    }
}
