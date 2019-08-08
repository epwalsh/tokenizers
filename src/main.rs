use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::PathBuf;

use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

use tokenizers::logger::{ErrorKind, Logger};
use tokenizers::Tokenizer;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tokenize",
    about = "tokenize a text file line-by-line.",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    /// The file to read and tokenize line-by-line.
    input: PathBuf,
    #[structopt(parse(from_os_str))]
    /// An optional output file. Default is stdout.
    output: Option<PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    let tokenizer = Tokenizer::english();
    let mut logger = Logger::new();

    // Initialize input file handle.
    let input_file = File::open(&opt.input)
        .with_context(|_| logger.failure(ErrorKind::FileRead))?;
    let input_handle = BufReader::new(input_file);

    // Initialize output file handle (default to stdout if no path was given).
    let mut output_handle = BufWriter::new(match &opt.output {
        Some(path) => Box::new(
            File::create(path).with_context(|_| logger.failure(ErrorKind::FileWrite))?,
        ) as Box<Write>,
        None => Box::new(io::stdout()) as Box<Write>,
    });

    // Iter through lines in the input file while tokenizing and writing to output file.
    for line in input_handle.lines() {
        let line = line.with_context(|_| logger.failure(ErrorKind::LineRead))?;
        let tokens = tokenizer.tokenize(line.as_str());
        writeln!(output_handle, "{:?}", tokens)
            .with_context(|_| logger.failure(ErrorKind::LineWrite))?;
        logger.update(tokens.len());
    }
    logger.finish();

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run_ok() {
        assert_eq!(2, 2);
    }
}
