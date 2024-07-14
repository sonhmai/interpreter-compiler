use std::io::{stderr, Write};

use crate::token::{Token, TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    line: usize,
    /// Current index of the char in source being processed.
    /// This is needed as a struct state to be used in scanning functions for different tokens.
    start_idx: usize,
    current_idx: usize,
    /// whether we had any tokenizing error
    pub error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            line: 1,
            start_idx: 0,
            current_idx: 0,
            error: false,
        }
    }

    pub fn scan(&mut self) -> () {
        let source = self.source.clone();
        // let mut chars = source.char_indices().peekable();
        let mut chars: Vec<char> = source.chars().collect();
        let mut ch: char;
        loop {
            if self.start_idx >= chars.len() {
                break;
            }
            ch = chars[self.start_idx];
            // println!("char {ch}, start {}, current {}", self.start_idx, self.current_idx);
            match ch {
                // single char tokens
                '(' => self.push_single_token(TokenType::LeftParen, ch.to_string()),
                ')' => self.push_single_token(TokenType::RightParen, ch.to_string()),
                '{' => self.push_single_token(TokenType::LeftBrace, ch.to_string()),
                '}' => self.push_single_token(TokenType::RightBrace, ch.to_string()),
                '*' => self.push_single_token(TokenType::STAR, ch.to_string()),
                '.' => self.push_single_token(TokenType::DOT, ch.to_string()),
                ',' => self.push_single_token(TokenType::COMMA, ch.to_string()),
                '+' => self.push_single_token(TokenType::PLUS, ch.to_string()),
                ';' => self.push_single_token(TokenType::SEMICOLON, ch.to_string()),
                '-' => self.push_single_token(TokenType::MINUS, ch.to_string()),

                // tokens with can be single or double chars
                // for EQUAL (=), cannot decide right away, need to look at next char for == case
                '=' => match self.next_char_match('=', &chars) {
                    true => self.push_two_token(TokenType::EqualEqual, "==".to_string()),
                    false => self.push_single_token(TokenType::Equal, ch.to_string())
                }
                // for BANG (!), cannot decide right away, need to look at next char for != case
                '!' => match self.next_char_match('=', &chars) {
                    true => self.push_two_token(TokenType::BangEqual, "!=".to_string()),
                    false => self.push_single_token(TokenType::Bang, ch.to_string())
                }
                '<' => match self.next_char_match('=', &chars) {
                    true => self.push_two_token(TokenType::LessEqual, "<=".to_string()),
                    false => self.push_single_token(TokenType::Less, ch.to_string())
                }
                '>' => match self.next_char_match('=', &chars) {
                    true => self.push_two_token(TokenType::GreaterEqual, ">=".to_string()),
                    false => self.push_single_token(TokenType::Greater, ch.to_string())
                }
                '/' => {
                    match self.next_char_match('/', &chars) {
                        true => {
                            // next char is /, this line is a comment (//)
                            // -> ignore all chars until new line or eof
                            loop {
                                self.advance(1);
                                ch = self.current_char(&chars);
                                match ch {
                                    '\n' | '\0' => {
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        false => self.push_single_token(TokenType::Slash, ch.to_string())
                    }
                }
                '\t' | ' ' | '\r' => self.advance(1), // ignore whitespaces
                '\n' => {
                    self.line += 1;
                    self.advance(1);
                }
                // tokens with many chars
                '"' => { // string
                    let mut lexeme: String = String::new();
                    // not consume the starting quote ", go past it
                    loop {
                        self.advance(1);
                        let ch = self.current_char(&chars);
                        match ch {
                            '\n' | '\0' => {
                                writeln!(
                                    stderr(),
                                    "[line {}] Error: Unterminated string.",
                                    self.line
                                ).unwrap();
                                self.error = true;
                                break;
                            }
                            '"' => {
                                // println!("breaking: char {ch}, start {}, current {}", self.start_idx, self.current_idx);
                                // not pushing the ending quote ", just go past it
                                self.push_token(TokenType::String, lexeme);
                                self.start_idx = self.current_idx;
                                self.advance(1); // advance past the ending "
                                break;
                            }
                            _ => {
                                lexeme.push(ch);
                                continue;
                            }
                        }
                    }
                }
                ch if ch.is_ascii_digit() => {
                    let mut lexeme: String = String::new();
                    lexeme.push(ch);
                    loop {
                        self.advance(1);
                        if self.start_idx >= chars.len() {
                            break;
                        }
                        let ch = chars[self.start_idx];
                        match ch {
                            // consume all numbers
                            '0'..='9' => {
                                lexeme.push(ch);
                                // println!("consumed {ch}, lexeme {lexeme}");
                                continue;
                            }
                            _ => break
                        }
                    }
                    // next in case of a DOT, then need to check if the next char after the dot
                    // is a number.
                    //  If yes, continue to consumer numbers. -> add token
                    //  If no, add should not consume the dot. -> add token
                    let ch = self.current_char(&chars);
                    if ch == '.' {
                        if self.next_char(&chars).is_ascii_digit() {
                            lexeme.push(ch); // consume the dot
                            loop {
                                self.advance(1);
                                if self.current_idx >= chars.len() {
                                    break;
                                }
                                let ch = chars[self.current_idx];
                                match ch {
                                    // consume all numbers
                                    '0'..='9' => {
                                        lexeme.push(ch);
                                        // println!("consumed {ch}, lexeme {lexeme}");
                                        continue;
                                    }
                                    _ => break
                                }
                            }
                        }
                    }
                    self.push_token(TokenType::Number, lexeme);
                    self.start_idx = self.current_idx;
                },
                ch if ch.is_ascii_alphabetic() || ch == '_' => {
                    let mut lexeme: String = String::new();
                    lexeme.push(ch);
                    loop {
                        self.advance(1);
                        if self.start_idx >= chars.len() {
                            break;
                        }
                        let ch = chars[self.start_idx];
                        match ch {
                            // consume all
                            ch if ch.is_ascii_alphanumeric() || ch == '_'  => {
                                lexeme.push(ch);
                                // println!("consumed {ch}, lexeme {lexeme}");
                                continue;
                            }
                            _ => break
                        }
                    }
                    self.push_token(TokenType::Identifier, lexeme);
                    self.start_idx = self.current_idx;
                },
                _ => {
                    self.push_token(TokenType::Unknown, ch.to_string());
                    self.advance(1);
                }
            }
        }

        self.tokens.push(Token::eof(self.line));
    }

    /// advance the char pointers
    fn advance(&mut self, num_chars: usize) {
        self.current_idx += num_chars;
        self.start_idx += num_chars;
    }

    fn reset_char_pointers(&mut self) {
        self.current_idx = 0;
        self.start_idx = 0;
    }

    fn push_single_token(&mut self, token_type: TokenType, lexeme: String) {
        self.advance(1);
        self.push_token(token_type, lexeme);
    }

    fn push_two_token(&mut self, token_type: TokenType, lexeme: String) {
        self.advance(2);
        self.push_token(token_type, lexeme);
    }

    fn push_token(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token::new(token_type, lexeme, self.line))
    }

    fn current_char(&self, chars: &Vec<char>) -> char {
        if self.current_idx >= chars.len() {
            return '\0';
        } else {
            chars[self.current_idx]
        }
    }

    fn next_char(&self, chars: &Vec<char>) -> char {
        if self.start_idx + 1 >= chars.len() {
            return '\0';
        } else {
            chars[self.start_idx + 1]
        }
    }

    /// look forward 2 chars
    fn peek_two(&self, chars: &Vec<char>) -> char {
        if self.start_idx + 2 >= chars.len() {
            return '\0';
        } else {
            chars[self.start_idx + 2]
        }
    }

    fn next_char_match(&mut self, ch: char, chars: &Vec<char>) -> bool {
        if self.start_idx + 1 >= chars.len() {
            return false;
        }
        if chars[self.start_idx + 1] != ch {
            return false;
        }
        return true;
    }
}

