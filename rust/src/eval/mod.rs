pub use value::Value;

use crate::ast::{Expression, Node};
use crate::ast::Statement;
use crate::eval::environment::Environment;

mod value;
pub mod environment;

pub fn eval(node: &Node, environment: &mut Environment) -> Value {
    match node {
        Node::Expression(expression) => eval_expression(expression),
        Node::Statement(statement) => eval_statement(statement, environment),
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

fn eval_statement(statement: &Statement, env: &mut Environment) -> Value {
    match statement {
        Statement::LetStatement(stmt) => {
            let value = eval_expression(&stmt.expression);
            let name = &stmt.identifier.name;
            env.set(name, value);
            Value::Null
        },
        _ => Value::Null
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Identifier, Node, Statement};
    use crate::ast::Expression::{BooleanExpression, IntegerLiteralExpression};
    use crate::ast::Node::Program;
    use crate::ast::statement::LetStatement;
    use crate::eval::{eval, Value};
    use crate::eval::environment::Environment;

    fn assert_value(node: Node, expected_value: Value) {
        let mut env = Environment::new();
        let value = eval(&node, &mut env);
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
            Program {
                statements: vec![
                    Statement::LetStatement(
                        LetStatement::new(
                            Identifier { name: "x".to_string() },
                            IntegerLiteralExpression(42),
                        )
                    ),
                ]
            },
            Value::Null,
        );

        // TODO test unbound identifier


    }
}