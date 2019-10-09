use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::str::SplitWhitespace;
use std::time;

use exitfailure::ExitFailure;
use failure::ResultExt;
use indicatif::ProgressBar;
use structopt::StructOpt;

use tokenizers::{Token, Tokenizer};

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
    let mut logger = Logger::new();
    let tokenizer = WhitespaceTokenizer::default();

    // Initialize input file handle.
    let input_file = File::open(&opt.input)
        .with_context(|e| format!("An error occured reading the input file: {}", e))?;
    let input_handle = BufReader::new(input_file);

    // Initialize output file handle (default to stdout if no path was given).
    let mut output_handle = BufWriter::new(match &opt.output {
        Some(path) => Box::new(
            File::create(path)
                .with_context(|e| format!("An error occured opening the output file: {}", e))?,
        ) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>,
    });

    // Iter through lines in the input file while tokenizing and writing to output file.
    let mut n_tokens: usize;
    for line in input_handle.lines() {
        let line = line?;
        n_tokens = 0;
        for token in tokenizer.tokenize(line.as_str()) {
            write!(output_handle, "{}\t", token.as_str())?;
            n_tokens += 1;
        }
        writeln!(output_handle)?;
        logger.update(n_tokens);
    }
    logger.finish();

    Ok(())
}

pub struct WhitespaceIterator<'a> {
    splitter_iter: SplitWhitespace<'a>,
}

impl<'a> WhitespaceIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        WhitespaceIterator {
            splitter_iter: input.split_whitespace(),
        }
    }
}

impl<'a> Iterator for WhitespaceIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        match self.splitter_iter.next() {
            Some(term) => Some(Token::from(term)),
            None => None,
        }
    }
}

#[derive(Default)]
pub struct WhitespaceTokenizer;

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
    type TokenIter = WhitespaceIterator<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        WhitespaceIterator::new(input)
    }
}

struct Logger {
    pb: ProgressBar,
    line_count: usize,
    token_count: usize,
    start_time: time::Instant,
    update_interval: usize,
}

impl Logger {
    fn new() -> Logger {
        let pb = ProgressBar::new_spinner();
        let line_count: usize = 0;
        let token_count: usize = 0;
        let start_time = time::Instant::now();
        let update_interval: usize = 100_000;
        Logger {
            pb,
            line_count,
            token_count,
            start_time,
            update_interval,
        }
    }

    fn tokens_per_second(&self) -> usize {
        let elapsed_nanoseconds: usize = self.start_time.elapsed().subsec_nanos() as usize;
        if elapsed_nanoseconds > 0 {
            (1_000_000_000 * self.token_count) / elapsed_nanoseconds
        } else {
            0
        }
    }

    fn update(&mut self, token_count: usize) {
        self.line_count += 1;
        self.token_count += token_count;
        if self.line_count % self.update_interval == 0 {
            let tokens_per_second = self.tokens_per_second();
            if tokens_per_second > 0 {
                self.pb.set_message(
                    format!(
                        "processed {} tokens ({} tokens per second)",
                        self.token_count, tokens_per_second
                    )
                    .as_str(),
                );
            } else {
                self.pb.inc(1);
            }
        }
    }

    fn finish(&self) {
        let tokens_per_second = self.tokens_per_second();
        if tokens_per_second > 0 {
            self.pb.finish_with_message(
                format!(
                    "processed {} tokens ({} tokens per second)",
                    self.token_count, tokens_per_second
                )
                .as_str(),
            );
        } else {
            self.pb.finish_with_message("done");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_tokenizer() {
        let s = "Hello, World!";
        let tokenizer = WhitespaceTokenizer::default();
        let tokens: Vec<Token> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens.len(), 2);
    }
}
