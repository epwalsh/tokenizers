use std::borrow::Cow;

/// Encapsulates the string term of a token.
///
/// The term that the token encapsulates is wrapped in a `Cow` (copy-on-write),
/// so that the string is only copied if it is modified.
///
/// # Examples
///
/// ```
/// let s = "Hello";  // note this is immutable.
/// let mut token = tokenizers::Token::from(s);
/// assert_eq!(token.term(), s);
/// token.term.to_mut().make_ascii_lowercase();
/// assert_eq!(token.term(), "hello");
/// ```
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
    fn test_token() {
        let s = "Hello";
        let mut token = Token::from(s);
        assert_eq!(token.term(), s);
        token.term.to_mut().make_ascii_lowercase();
        assert_ne!(token.term(), s);
        assert_eq!(token.term(), "hello");
    }
}
