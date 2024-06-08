pub(crate) mod statement;

#[derive(Debug)]
pub enum Node {
    Program {
        statements: Vec<dyn Statement>,
    },
    Statement(dyn Statement),
    Expression(Expression),
}

pub trait Statement {}

#[derive(Debug)]
pub enum Expression {
    BooleanExpression(bool),
    IntegerLiteralExpression(i32),
}

// Identifier represents variable like "x" in `let x = 5;`
pub struct Identifier {
    pub name: String,
}