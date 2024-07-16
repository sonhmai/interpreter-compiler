use crate::ast::statement::LetStatement;

pub mod statement;

#[derive(Debug)]
pub enum Node {
    Program {
        statements: Vec<Statement>,
    },
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ConstStatement,
    ReturnStatement,
}

#[derive(Debug)]
pub enum Expression {
    BooleanExpression(bool),
    IntegerLiteralExpression(i32),
}

// Identifier represents variable like "x" in `let x = 5;`
#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}
