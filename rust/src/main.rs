use std::sync::Arc;
use compiler::eval::environment::Environment;
use compiler::eval::eval;
use compiler::lexer::Lexer;
use compiler::parser::Parser;

fn main() {

    let code = "let x = 5";
    let lexer = Arc::new(Lexer::new(code));
    let parser = Parser::new(lexer.clone());
    let ast = parser.parse();
    let mut env = Environment::new();
    let value = eval(&ast, &mut env);

    println!("AST: {ast:?}");
    println!("evaluated value: {value:?}");
}
