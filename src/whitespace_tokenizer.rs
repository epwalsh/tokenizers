use std::str::SplitWhitespace;

use crate::{Token, Tokenizer};

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

pub struct WhitespaceTokenizer;

impl WhitespaceTokenizer {
    pub fn new() -> Self {
        WhitespaceTokenizer {}
    }
}

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
    type TokenIter = WhitespaceIterator<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        WhitespaceIterator::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_tokenizer() {
        let s = "Hello, World!";
        let tokenizer = WhitespaceTokenizer::new();
        let tokens: Vec<Token> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens.len(), 2);
    }
}
