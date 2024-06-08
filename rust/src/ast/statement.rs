use crate::ast::{Expression, Identifier};

#[derive(Debug)]
pub struct LetStatement {
    pub identifier: Identifier,
    pub expression: Expression,
}

impl LetStatement {
    pub fn new(identifier: Identifier, expression: Expression) -> Self {
        LetStatement {
            identifier,
            expression,
        }
    }
}
