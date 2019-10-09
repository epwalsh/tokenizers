use std::borrow::Cow;
use std::cmp::{Eq, PartialEq};
use std::fmt;

/// Encapsulates the string term of a token.
///
/// The term that the token encapsulates is wrapped in a `Cow` (copy-on-write),
/// so that the string slice is only copied if it is modified.
pub struct Token<'a> {
    term: Cow<'a, str>,

    /// The index of the token within the original un-tokenized string slice.
    pub offset: Option<usize>,
}

impl<'a> Token<'a> {
    /// Create a new `Token` object.
    pub fn new(term: &'a str, offset: Option<usize>) -> Self {
        Token {
            term: Cow::Borrowed(term),
            offset,
        }
    }

    /// Initializes a `Token` from a string slice.
    pub fn from(term: &'a str) -> Self {
        Token {
            term: Cow::Borrowed(term),
            offset: None,
        }
    }

    /// Extracts the string slice contained in the `Token`.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = "Hello";
    /// let token = tokenizers::Token::from(s);
    /// assert_eq!(token.as_str(), s);
    /// ```
    pub fn as_str(&self) -> &str {
        self.term.as_ref()
    }

    /// Returns a mutable reference to the contained string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut token = tokenizers::Token::from("Hello");
    /// token.as_mut_str().make_ascii_lowercase();
    /// assert_eq!(token.as_str(), "hello");
    /// ```
    pub fn as_mut_str(&mut self) -> &mut str {
        self.term.to_mut()
    }

    /// Clones the contained string slice if it is not already owned (i.e. if it has not already
    /// been modified) and returns it as a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut token = tokenizers::Token::from("Hello");
    /// assert_eq!(token.into_string(), String::from("Hello"));
    /// ```
    pub fn into_string(self) -> String {
        self.term.into_owned()
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> Eq for Token<'a> {}

impl<'a> PartialEq<String> for Token<'a> {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> PartialEq<&str> for Token<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

/// A tokenizer is simply a struct implementing a `tokenize` function that takes a
/// string slice and returns an iterator of `Token`s.
pub trait Tokenizer<'a> {
    type TokenIter: Iterator<Item = Token<'a>>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_change_case() {
        let s = "Hello";
        let mut token = Token::from(s);
        assert_eq!(token.as_str(), s);
        token.term.to_mut().make_ascii_lowercase();
        assert_ne!(token.as_str(), s);
        assert_eq!(token.as_str(), "hello");
    }

    #[test]
    fn test_token_display() {
        let s = "Hello";
        let token = Token::from(s);
        assert_eq!(format!("{}", token), "Hello");
    }

    #[test]
    fn test_token_comp_token() {
        let token1 = Token::from("foo");
        let token2 = Token::from("foo");
        let token3 = Token::from("bar");
        assert_eq!(token1 == token2, true);
        assert_eq!(token1 == token3, false);
        assert_eq!(token1 != token3, true);
    }

    #[test]
    fn test_token_comp_string() {
        let token = Token::from("foo");
        assert_eq!(token == String::from("foo"), true);
        assert_eq!(token == String::from("bar"), false);
        assert_eq!(token != String::from("bar"), true);
    }

    #[test]
    fn test_token_comp_string_slice() {
        let token = Token::from("foo");
        assert_eq!(token == "foo", true);
        assert_eq!(token == "bar", false);
        assert_eq!(token != "bar", true);
    }
}
