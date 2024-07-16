use std::sync::Arc;
use crate::ast::Node;
use crate::lexer::Lexer;

#[derive(Debug)]
pub struct Parser {
    lexer: Arc<Lexer>,
}

impl Parser {
    pub fn new(lexer: Arc<Lexer>) -> Parser {
        Parser {
            lexer
        }
    }

    pub fn parse(&self) -> Node {
        let program = Node::Program {
            statements: vec![]
        };

        program
    }
}
