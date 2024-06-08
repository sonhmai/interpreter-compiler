// Don't attach this module yet as using trait we must deal with
// compiler error "size must be known at compile time" and
// for dyn we do not know that.

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