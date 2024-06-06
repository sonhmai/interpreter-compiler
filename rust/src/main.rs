use std::sync::Arc;
use compiler::lexer::Lexer;
use compiler::parser::Parser;

fn main() {

    let code = "let x = 5";
    let lexer = Arc::new(Lexer::new(code));
    let parser = Parser::new(lexer.clone());

    println!("{parser:?}")
}
