use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string()
        }
    }

    pub fn next_token(&mut self) -> Token {
        todo!()
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
    fn test_lexer_should_parse_correct_tokens() {
        assert_correct_tokens(
            "let s=5;",
            vec![
                Token::Keyword("let".to_string()),
                Token::Identifier("s".to_string()),
                Token::EqualSign,
                Token::Int(5),
                Token::EOF,
            ]
        );
    }
}
