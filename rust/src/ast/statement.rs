use crate::ast::{Expression, Identifier, Statement};


impl Statement for LetStatement {}


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

pub struct ConstStatement {}

pub struct ReturnStatement {}