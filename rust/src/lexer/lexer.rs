use std::str::{CharIndices, Chars};
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    chars: Chars,
    idx: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            indices: input.char_indices(),
            idx: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        // why not use self.input[self.idx]? because indexing string by bytes in
        // not correct in Rust as string is UTF-8 encoded, 1 char can be >1 bytes.
        if self.idx >= self.input.len() {
            return Token::EOF
        }

        Token::EqualSign
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    fn assert_correct_tokens(code: &str, expected_tokens: Vec<Token>) {
        let mut lexer = Lexer::new(code);
        let mut token;
        for expected in expected_tokens {
            token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_lexer_should_parse_eof() {
        assert_correct_tokens(
            "",
            vec![
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_lexer_should_parse_semicolon() {
        assert_correct_tokens(
            ";",
            vec![
                Token::Semicolon,
                Token::EOF,
            ]
        );

        assert_correct_tokens(
            "();",
            vec![
                Token::Semicolon,
                Token::EOF,
            ]
        );
    }


    #[test]
    fn test_lexer_should_parse_correct_tokens() {
        assert_correct_tokens(
            "let s=5;",
            vec![
                Token::Keyword("let".to_string()),
                Token::Identifier("s".to_string()),
                Token::EqualSign,
                Token::Int(5),
                Token::Semicolon,
                Token::EOF,
            ]
        );
    }
}
