mod value;

pub use value::Value;

use crate::ast::{Expression, Node, Statement};

pub fn eval(node: &Node) -> Value {
    match node {
        Node::Expression(expression) => eval_expression(expression),
        Node::Statement(statement) => eval_statement(statement),
        _ => Value::Null
    }
}

fn eval_expression(expression: &Expression) -> Value {
    match expression {
        Expression::IntegerLiteralExpression(i) => Value::Integer(*i),
        Expression::BooleanExpression(boolean) => Value::Boolean(*boolean),
        _ => Value::Null
    }
}

fn eval_statement(statement: &Statement) -> Value {
    match statement {
        _ => Value::Null
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression::{BooleanExpression, IntegerLiteralExpression};
    use crate::ast::{Expression, Node};
    use crate::eval::{eval, Value};

    fn assert_value(node: Node, expected_value: Value) {
        let value = eval(&node);
        assert_eq!(value, expected_value)
    }

    #[test]
    fn test_eval_int() {
        assert_value(
            Node::Expression(IntegerLiteralExpression(42)),
            Value::Integer(42),
        )
    }

    #[test]
    fn test_eval_bool() {
        assert_value(
            Node::Expression(BooleanExpression(true)),
            Value::Boolean(true),
        );
        assert_value(
            Node::Expression(BooleanExpression(false)),
            Value::Boolean(false),
        );
    }

    #[test]
    fn test_eval_let() {
        assert_value(
            Node::Expression(BooleanExpression(true)),
            Value::Boolean(true),
        );
    }
}