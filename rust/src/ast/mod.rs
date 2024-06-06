

pub enum Node {
    Program {
        statements: Vec<Statement>,
    }
}

pub enum Statement {
    LetStatement,
    ConstStatement,
    ReturnStatement,
}

pub enum Expression {
    Identifier {
        name: String,
    },
    BooleanExpression,
    IntegerLiteralExpression(i32),
}