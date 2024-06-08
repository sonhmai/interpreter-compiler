use crate::ast::{Expression, Identifier};

#[derive(Debug)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl LetStatement {
    pub fn new(identifier: Identifier, expression: Expression) -> Self {
        LetStatement {
            name: identifier,
            value: expression,
        }
    }
}
