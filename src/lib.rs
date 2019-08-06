extern crate regex;

use regex::{Matches, Regex};

#[derive(Debug)]
pub struct Tokenizer {
    token_re: Regex,
    infix_re: Regex,
}

impl Tokenizer {
    pub fn new(token_pattern: &str, infix_pattern: &str) -> Tokenizer {
        let token_re = Regex::new(token_pattern).unwrap();
        let infix_re = Regex::new(infix_pattern).unwrap();
        Tokenizer {
            token_re: token_re,
            infix_re: infix_re,
        }
    }

    pub fn english() -> Tokenizer {
        let token_pattern = r"^(https?://.*)$";
        let infix_pattern = r"[?,!-.*+]";
        Tokenizer::new(token_pattern, infix_pattern)
    }

    pub fn tokenize<'a>(&self, s: &'a str) -> Vec<&'a str> {
        let mut final_tokens: Vec<&str> = Vec::new();
        let token_iterator = s.split_whitespace();
        let mut sub_token_index: usize;
        for token in token_iterator {
            // Check for a `token_re` match, which matches patterns that should
            // be treated as whole tokens.
            if self.token_re.is_match(token) {
                final_tokens.push(token);
                continue;
            }
            // Otherwise we search for infixes next and split up the token
            // into sub-tokens separated by infix patterns.
            sub_token_index = 0;
            for infix_match in self.find_infix(token) {
                let sub_token = token.get(sub_token_index..infix_match.start()).unwrap();
                let infix = token.get(infix_match.start()..infix_match.end()).unwrap();
                sub_token_index = infix_match.end();
                if sub_token.len() > 0 {
                    final_tokens.push(sub_token);
                }
                final_tokens.push(infix);
            }
            let sub_token = token.get(sub_token_index..token.len()).unwrap();
            if sub_token.len() > 0 {
                final_tokens.push(sub_token);
            }
        }
        final_tokens
    }

    pub fn find_infix<'a, 'b>(&'a self, token: &'b str) -> Matches<'a, 'b> {
        self.infix_re.find_iter(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokenizer = Tokenizer::english();
        let s = String::from("Héllo,, World!-huh?\r\n😃 my website is https://epwalsh.com");
        let tokens = tokenizer.tokenize(&s);
        assert_eq!(
            tokens,
            vec![
                "Héllo",
                ",",
                ",",
                "World",
                "!",
                "-",
                "huh",
                "?",
                "😃",
                "my",
                "website",
                "is",
                "https://epwalsh.com",
            ]
        );
    }

    #[test]
    fn test_tokenize_punctuation() {
        let tokenizer = Tokenizer::english();
        let s = ".?-+*";
        let tokens = tokenizer.tokenize(s);
        let expected: Vec<&str> = s.split("").collect();
        // NOTE: `expected` will have an empty string at the beginning and end,
        // so we only use the interior elements for comparison.
        assert_eq!(tokens[..], expected[1..expected.len() - 1]);
    }
}
