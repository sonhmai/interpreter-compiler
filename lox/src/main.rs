use std::env;
use std::fs;
use std::io::{self, Write};

use scanning::Scanner;

use crate::token::TokenType;

mod scanning;
pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(file_contents);
            scanner.scan();

            let mut error = false;
            for token in scanner.tokens {
                match token.token_type {
                    TokenType::Unknown => {
                        error = true;
                        writeln!(
                            io::stderr(),
                            "[line {}] Error: Unexpected character: {}",
                            token.line,
                            token.lexeme
                        ).unwrap()
                    }
                    _ =>
                        println!("{token}")
                }
            }
            if error | scanner.error {
                std::process::exit(65)
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
