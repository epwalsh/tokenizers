use std::borrow::Cow;
use std::str::SplitWhitespace;

pub struct Token<'a> {
    pub term: Cow<'a, str>,
}

impl<'a> Token<'a> {
    pub fn from(term: &'a str) -> Self {
        Token {
            term: Cow::Borrowed(term),
        }
    }

    pub fn term(&self) -> &str {
        self.term.as_ref()
    }
}

pub trait Tokenizer<'a> {
    type TokenIter: Iterator<Item = Token<'a>>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}

pub struct SimpleTokenIterator<'a> {
    splitter_iter: SplitWhitespace<'a>,
}

impl<'a> SimpleTokenIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        SimpleTokenIterator {
            splitter_iter: input.split_whitespace(),
        }
    }
}

impl<'a> Iterator for SimpleTokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        match self.splitter_iter.next() {
            Some(term) => Some(Token::from(term)),
            None => None,
        }
    }
}

pub struct SimpleTokenizer;

impl SimpleTokenizer {
    pub fn new() -> Self {
        SimpleTokenizer {}
    }
}

impl<'a> Tokenizer<'a> for SimpleTokenizer {
    type TokenIter = SimpleTokenIterator<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        SimpleTokenIterator::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let s = "Hello";
        let mut token = Token::from(s);
        assert_eq!(token.term(), s);
        token.term.to_mut().make_ascii_lowercase();
        assert_ne!(token.term(), s);
        assert_eq!(token.term(), "hello");
    }

    #[test]
    fn test_simple_tokenizer() {
        let s = "Hello, World!";
        let tokenizer = SimpleTokenizer::new();
        let tokens: Vec<Token> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens.len(), 2);
    }
}
