extern crate regex;

use regex::{Matches, Regex};


#[derive(Debug)]
pub struct Tokenizer {
    token_re: Regex,
    infix_re: Regex,
    prefix_re: Regex,
    suffix_re: Regex,
}

impl Tokenizer {
    pub fn new(
        token_pattern: &str,
        infix_pattern: &str,
        prefix_pattern: &str,
        suffix_pattern: &str,
    ) -> Tokenizer {
        let token_re = Regex::new(token_pattern).unwrap();
        let infix_re = Regex::new(infix_pattern).unwrap();
        let prefix_re = Regex::new(prefix_pattern).unwrap();
        let suffix_re = Regex::new(suffix_pattern).unwrap();
        Tokenizer {
            token_re,
            infix_re,
            prefix_re,
            suffix_re,
        }
    }

    pub fn english() -> Tokenizer {
        let token_pattern = r"^(https?://.*)$";
        let infix_pattern = r"[?,!.*+-]";
        let prefix_pattern = r"^(\$)";
        let suffix_pattern = r"('m|'t|'d|'s|%)$";
        Tokenizer::new(token_pattern, infix_pattern, prefix_pattern, suffix_pattern)
    }

    fn add_affixes<'a>(&self, tokens: &mut Vec<&'a str>, token: &'a str) {
        let token_size = token.len();
        if token_size > 0 {
            let prefix_size = match self.find_prefix(token) {
                Some(size) => size,
                None => 0,
            };
            let suffix_size = match self.find_suffix(token) {
                Some(size) => size,
                None => 0,
            };
            let mut middle_end = token_size - suffix_size;
            // If the suffix starts before the prefix ends, just ignore the suffix.
            if middle_end < prefix_size {
                middle_end = token_size
            };
            if middle_end > prefix_size {
                let middle = token.get(prefix_size..middle_end).unwrap();
                tokens.push(middle);
            }
            if prefix_size > 0 {
                let prefix = token.get(0..prefix_size).unwrap();
                tokens.push(prefix);
            }
            if token_size > middle_end {
                let suffix = token.get(middle_end..token_size).unwrap();
                tokens.push(suffix);
            }
        }
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
                self.add_affixes(&mut final_tokens, sub_token);
                final_tokens.push(infix);
            }
            if sub_token_index < token.len() {
                let sub_token = token.get(sub_token_index..token.len()).unwrap();
                self.add_affixes(&mut final_tokens, sub_token);
            }
        }
        final_tokens
    }

    pub fn find_infix<'a, 'b>(&'a self, token: &'b str) -> Matches<'a, 'b> {
        self.infix_re.find_iter(token)
    }

    pub fn find_prefix(&self, token: &str) -> Option<usize> {
        match self.prefix_re.find(token) {
            Some(mat) => {
                return Some(mat.end() - mat.start());
            }
            None => {
                return None;
            }
        }
    }

    pub fn find_suffix(&self, token: &str) -> Option<usize> {
        match self.suffix_re.find(token) {
            Some(mat) => {
                return Some(mat.end() - mat.start());
            }
            None => {
                return None;
            }
        }
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

    #[test]
    fn test_tokenize_empty_string() {
        let tokenizer = Tokenizer::english();
        let s = "";
        let tokens = tokenizer.tokenize(s);
        let expected: Vec<&str> = Vec::new();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_apostrophes() {
        let tokenizer = Tokenizer::english();
        let s = "I'm";
        let tokens = tokenizer.tokenize(s);
        assert_eq!(tokens, vec!["I", "'m"]);
    }

    #[test]
    fn test_tokenize_find_prefix() {
        let tokenizer = Tokenizer::english();
        let token = "$2";
        let prefix_len = tokenizer.find_prefix(token).unwrap();
        assert_eq!(prefix_len, 1);
    }

    #[test]
    fn test_tokenize_find_suffix() {
        let tokenizer = Tokenizer::english();
        let token = "I'm";
        let suffix_len = tokenizer.find_suffix(token).unwrap();
        assert_eq!(suffix_len, 2);
    }
}
