use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TokenType {
    Unknown,

    EOF, // end of file
    // punctuates
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    STAR,
    DOT,
    COMMA,
    PLUS,
    MINUS,
    SEMICOLON,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
    // tokens with >=1 chars
    String,
    Number,
    Identifier,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Unknown => write!(f, "Unexpected character"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::STAR => write!(f, "STAR"),
            TokenType::DOT => write!(f, "DOT"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
        }
    }
}

/// Token is <token_type> <lexeme> <literal>
///     VAR var null
///     SEMICOLON ; null
///     LEFTPAREN ( null
/// output format matches the spec in the book's repository
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token { token_type, lexeme, line }
    }

    pub fn eof(line: usize) -> Self {
        Token::new(TokenType::EOF, "".to_string(), line)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.token_type {
            TokenType::String => write!(f, "{} \"{}\" {}", self.token_type, self.lexeme, self.lexeme),
            TokenType::Number => write!(
                // {:?} to give at least 1 decimal denoting float: 3.0 for 3, 3.14 for 3.14
                f, "{} {} {:?}",
                self.token_type,
                self.lexeme,
                self.lexeme.parse::<f32>().unwrap(),
            ),
            _ => write!(f, "{} {} null", self.token_type, self.lexeme)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    #[test]
    fn parens_display_correctly() {
        let token = Token::new(TokenType::LeftParen, "(".to_string(), 1);
        assert_eq!(format!("{}", token), "LEFT_PAREN ( null");

        let token = Token::new(TokenType::RightParen, ")".to_string(), 1);
        assert_eq!(format!("{}", token), "RIGHT_PAREN ) null");
    }

    #[test]
    fn eof_display() {
        let token = Token::eof(1);
        // must have 2 spaces between EOF and null
        assert_eq!(format!("{}", token), "EOF  null");
    }

    fn assert_number(code: &str, expected_display: &str) {
        let token = Token::new(TokenType::Number, code.to_string(), 1);
        assert_eq!(format!("{}", token), expected_display);
    }

    #[test]
    fn display_number() {
        // https://github.com/munificent/craftinginterpreters/blob/01e6f5b8f3e5dfa65674c2f9cf4700d73ab41cf8/test/scanning/numbers.lox
        assert_number("123.456", "NUMBER 123.456 123.456");
        assert_number("123", "NUMBER 123 123.0");
    }
}
