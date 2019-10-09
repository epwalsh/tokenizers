use std::str::SplitWhitespace;

use tokenizers::{Token, Tokenizer};

struct WhitespaceIterator<'a> {
    splitter_iter: SplitWhitespace<'a>,
}

impl<'a> WhitespaceIterator<'a> {
    fn new(input: &'a str) -> Self {
        WhitespaceIterator {
            splitter_iter: input.split_whitespace(),
        }
    }
}

impl<'a> Iterator for WhitespaceIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        match self.splitter_iter.next() {
            Some(term) => {
                if term == "foo" {
                    Some(Token::from("bar")) // this just tests that we can completely replace a term.
                } else {
                    Some(Token::from(term))
                }
            }
            None => None,
        }
    }
}

#[derive(Default)]
struct WhitespaceTokenizer;

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
    type TokenIter = WhitespaceIterator<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        WhitespaceIterator::new(input)
    }
}

#[test]
fn test_tokenizer() {
    let tokenizer = WhitespaceTokenizer::default();
    let s = "Hello, World!";
    let tokens: Vec<Token> = tokenizer.tokenize(s).collect();
    assert_eq!(tokens.len(), 2);
}

#[test]
fn test_tokenizer_modify_term() {
    let tokenizer = WhitespaceTokenizer::default();
    let s = "foo bar";
    let tokens: Vec<Token> = tokenizer.tokenize(s).collect();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].as_str(), "bar");
}
