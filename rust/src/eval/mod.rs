mod value;
pub use value::Value;

use crate::ast::Node;

pub fn eval(node: &Node) -> Value {

    Value::Null
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression::IntegerLiteralExpression;
    use crate::ast::Node;
    use crate::eval::{eval, Value};

    #[test]
    fn test_eval_int() {
        let int_expr = Node::Expression(IntegerLiteralExpression(42));
        let value = eval(&int_expr);
        assert_eq!(value, Value::Integer(42))
    }
}