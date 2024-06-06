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

