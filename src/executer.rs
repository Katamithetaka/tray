use crate::parser;
#[derive(Debug)]
pub enum Result {
    Number(parser::Number),
    Char(char),
    String(String),
}

pub fn execute(expr: &parser::Expression) -> Result {
    match expr {
        parser::Expression::UnaryOperation { operator, expr } => match operator {
            parser::UnaryOperator::Plus => {
                let result = execute(expr);

                match result {
                    Result::Number(_) => result,
                    _ => panic!("tried to execute unary plus operator on non number expression."),
                }
            }
            parser::UnaryOperator::Minus => {
                let result = execute(expr);
                match result {
                    Result::Number(result) => match result {
                        parser::Number::Float32(v) => {
                            Result::Number(parser::Number::Float32(-1. * v))
                        }
                        parser::Number::Float64(v) => {
                            Result::Number(parser::Number::Float64(-1. * v))
                        }
                        parser::Number::Int32(v) => Result::Number(parser::Number::Int32(-1 * v)),
                        parser::Number::Int64(v) => Result::Number(parser::Number::Int64(-1 * v)),
                        parser::Number::Int128(v) => Result::Number(parser::Number::Int128(-1 * v)),
                    },
                    _ => panic!("tried to apply unary minus operator to a non number expression"),
                }
            }
        },
        parser::Expression::BinaryOperation {
            operator,
            left,
            right,
        } => {
            let left = execute(left);
            let right = execute(right);

            if let (Result::Number(left), Result::Number(right)) = (left, right) {
                if left.is_floating_point(&right) {
                    match operator {
                        parser::BinaryOperator::Plus => {
                            Result::Number(parser::Number::Float64(left.as_f64() + right.as_f64()))
                        }
                        parser::BinaryOperator::Minus => {
                            Result::Number(parser::Number::Float64(left.as_f64() - right.as_f64()))
                        }
                        parser::BinaryOperator::Divide => {
                            Result::Number(parser::Number::Float64(left.as_f64() / right.as_f64()))
                        }
                        parser::BinaryOperator::Multiply => {
                            Result::Number(parser::Number::Float64(left.as_f64() * right.as_f64()))
                        }
                    }
                } else {
                    match operator {
                        parser::BinaryOperator::Plus => {
                            Result::Number(parser::Number::Int128(left.as_i128() + right.as_i128()))
                        }
                        parser::BinaryOperator::Minus => {
                            Result::Number(parser::Number::Int128(left.as_i128() - right.as_i128()))
                        }
                        parser::BinaryOperator::Divide => {
                            Result::Number(parser::Number::Int128(left.as_i128() / right.as_i128()))
                        }
                        parser::BinaryOperator::Multiply => {
                            Result::Number(parser::Number::Int128(left.as_i128() * right.as_i128()))
                        }
                    }
                }
            } else {
                panic!("tried to do a binary operation on non number expressions");
            }
        }
        parser::Expression::ParenthesisExpression(expr) => execute(expr),
        parser::Expression::Number(value) => Result::Number(value.clone()),
        parser::Expression::Char(char) => Result::Char(*char),
        parser::Expression::String(string) => Result::String(string.clone()),
    }
}
