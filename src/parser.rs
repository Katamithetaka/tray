use std::{
    iter::{Enumerate, Peekable},
    slice::Iter,
};

use crate::lexer::{Token, TokenList};

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

impl BinaryOperator {
    pub fn priority(&self) -> u8 {
        match self {
            BinaryOperator::Plus => 1,
            BinaryOperator::Minus => 0,
            BinaryOperator::Divide => 2,
            BinaryOperator::Multiply => 3,
        }
    }
}

impl Ord for BinaryOperator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Float32(f32),
    Float64(f64),
    Int32(i32),
    Int64(i64),
    Int128(i128),
}

#[allow(dead_code)]
impl Number {
    pub fn as_f64(&self) -> f64 {
        match self {
            Number::Float32(v) => *v as f64,
            Number::Float64(v) => *v as f64,
            Number::Int32(v) => *v as f64,
            Number::Int64(v) => *v as f64,
            Number::Int128(v) => *v as f64,
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            Number::Float32(v) => *v as f32,
            Number::Float64(v) => *v as f32,
            Number::Int32(v) => *v as f32,
            Number::Int64(v) => *v as f32,
            Number::Int128(v) => *v as f32,
        }
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            Number::Float32(v) => *v as i32,
            Number::Float64(v) => *v as i32,
            Number::Int32(v) => *v as i32,
            Number::Int64(v) => *v as i32,
            Number::Int128(v) => *v as i32,
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            Number::Float32(v) => *v as i64,
            Number::Float64(v) => *v as i64,
            Number::Int32(v) => *v as i64,
            Number::Int64(v) => *v as i64,
            Number::Int128(v) => *v as i64,
        }
    }

    pub fn as_i128(&self) -> i128 {
        match self {
            Number::Float32(v) => *v as i128,
            Number::Float64(v) => *v as i128,
            Number::Int32(v) => *v as i128,
            Number::Int64(v) => *v as i128,
            Number::Int128(v) => *v as i128,
        }
    }

    pub fn is_floating_point(&self, other: &Self) -> bool {
        if let (Number::Float32(_) | Number::Float64(_), _)
        | (_, Number::Float32(_) | Number::Float64(_)) = (self, other)
        {
            true
        } else {
            false
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression {
    UnaryOperation {
        operator: UnaryOperator,
        expr: Box<Expression>,
    },
    BinaryOperation {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    ParenthesisExpression(Box<Expression>),
    Number(Number),
    Char(char),
    String(String),
}

impl Expression {
    pub fn unwrap_content(&self) -> Expression {
        if let Self::ParenthesisExpression(expr) = self {
            expr.unwrap_content()
        } else {
            self.clone()
        }
    }
}

fn default_parse(iterator: &mut Peekable<Enumerate<Iter<Token>>>) -> Option<Expression> {
    if let Some((_index, token)) = iterator.next() {
        Some(match token {
            Token::Plus => parse_unary(iterator, UnaryOperator::Plus)?,
            Token::Minus => parse_unary(iterator, UnaryOperator::Minus)?,
            Token::Multiply => panic!(""),
            Token::Divide => panic!(""),
            Token::I32(value) => parse_number(iterator, Number::Int32(*value))?,
            Token::I64(value) => parse_number(iterator, Number::Int64(*value))?,
            Token::I128(value) => parse_number(iterator, Number::Int128(*value))?,
            Token::F32(value) => parse_number(iterator, Number::Float32(*value))?,
            Token::F64(value) => parse_number(iterator, Number::Float64(*value))?,
            Token::String(value) => Expression::String(value.clone()),
            Token::Char(value) => Expression::Char(*value),
            Token::LParenthesis => parse_lparen(iterator)?,
            Token::RParenthesis => panic!(""),
        })
    } else {
        None
    }
}

fn parse_lparen(iterator: &mut Peekable<Enumerate<Iter<Token>>>) -> Option<Expression> {
    let expression = default_parse(iterator)?;
    if let Some((_, Token::RParenthesis)) = iterator.next() {
        let expression = Expression::ParenthesisExpression(Box::new(expression));
        let next_token = iterator.peek().cloned();
        if let Some((_, next_token)) = next_token {
            match next_token {
                Token::Plus => {
                    iterator.next().expect("iterator should still be valid");
                    parse_binary(iterator, expression, BinaryOperator::Plus)
                }
                Token::Minus => {
                    iterator.next().expect("iterator should still be valid");
                    parse_binary(iterator, expression, BinaryOperator::Minus)
                }
                Token::Multiply => {
                    iterator.next().expect("iterator should still be valid");
                    parse_binary(iterator, expression, BinaryOperator::Multiply)
                }
                Token::Divide => {
                    iterator.next().expect("iterator should still be valid");
                    parse_binary(iterator, expression, BinaryOperator::Divide)
                }
                Token::RParenthesis => Some(expression),
                _ => panic!("Unexpected token after parenthesis expression"),
            }
        } else {
            Some(expression)
        }
    } else {
        panic!("Unclosed parenthesis");
    }
}

fn parse_number(
    iterator: &mut Peekable<Enumerate<Iter<Token>>>,
    number: Number,
) -> Option<Expression> {
    if let Some((_, token)) = iterator.peek() {
        match token {
            Token::Plus => {
                iterator.next().expect("Iterator should still be valid");
                parse_binary(iterator, Expression::Number(number), BinaryOperator::Plus)
            }
            Token::Minus => {
                iterator.next().expect("Iterator should still be valid");
                parse_binary(iterator, Expression::Number(number), BinaryOperator::Minus)
            }
            Token::Multiply => {
                iterator.next().expect("Iterator should still be valid");
                parse_binary(
                    iterator,
                    Expression::Number(number),
                    BinaryOperator::Multiply,
                )
            }
            Token::Divide => {
                iterator.next().expect("Iterator should still be valid");
                parse_binary(iterator, Expression::Number(number), BinaryOperator::Divide)
            }

            Token::RParenthesis => Some(Expression::Number(number)),
            _ => panic!(""),
        }
    } else {
        Some(Expression::Number(number))
    }
}

fn parse_binary(
    iterator: &mut Peekable<Enumerate<Iter<Token>>>,
    expression: Expression,
    operator: BinaryOperator,
) -> Option<Expression> {
    if let Some(second_expression) = default_parse(iterator) {
        if let Expression::String(_) | Expression::Char(_) = second_expression.unwrap_content() {
            panic!("tried to add number to string or char")
        }
        if let Expression::BinaryOperation {
            operator: second_operator,
            left,
            right,
        } = second_expression.clone()
        {
            match operator.cmp(&second_operator) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    Some(Expression::BinaryOperation {
                        operator,
                        left: Box::new(expression),
                        right: Box::new(second_expression),
                    })
                }
                std::cmp::Ordering::Greater => Some(Expression::BinaryOperation {
                    operator: second_operator,
                    left: Box::new(Expression::BinaryOperation {
                        operator,
                        left: Box::new(expression),
                        right: left,
                    }),
                    right,
                }),
            }
        } else {
            Some(Expression::BinaryOperation {
                operator,
                left: Box::new(expression),
                right: Box::new(second_expression),
            })
        }
    } else {
        panic!("")
    }
}

fn parse_unary(
    iterator: &mut Peekable<Enumerate<Iter<Token>>>,
    operator: UnaryOperator,
) -> Option<Expression> {
    let expr;

    if let Some((_index, token)) = iterator.peek() {
        expr = match token {
            Token::F32(f) => {
                iterator.next().expect("Iterator should still be valid.");
                Expression::Number(Number::Float32(*f))
            }
            Token::F64(f) => {
                iterator.next().expect("Iterator should still be valid.");
                Expression::Number(Number::Float64(*f))
            }
            Token::I32(f) => {
                iterator.next().expect("Iterator should still be valid.");
                Expression::Number(Number::Int32(*f))
            }
            Token::I64(f) => {
                iterator.next().expect("Iterator should still be valid.");
                Expression::Number(Number::Int64(*f))
            }
            Token::I128(f) => {
                iterator.next().expect("Iterator should still be valid.");
                Expression::Number(Number::Int128(*f))
            }
            _ => default_parse(iterator)?,
        };

        if let Expression::String(_) | Expression::Char(_) = expr {
            panic!("")
        }

        let unary_expr = Expression::UnaryOperation {
            operator,
            expr: Box::new(expr),
        };

        if let Some((_, token)) = iterator.next() {
            match token {
                Token::Plus => parse_binary(iterator, unary_expr, BinaryOperator::Plus),
                Token::Minus => parse_binary(iterator, unary_expr, BinaryOperator::Minus),
                Token::Multiply => parse_binary(iterator, unary_expr, BinaryOperator::Multiply),
                Token::Divide => parse_binary(iterator, unary_expr, BinaryOperator::Divide),
                Token::RParenthesis => Some(unary_expr),
                _ => panic!(),
            }
        } else {
            Some(unary_expr)
        }
    } else {
        None
    }
}

pub fn parse(tokens: &TokenList) -> Option<Expression> {
    let mut iterator = tokens.iter().enumerate().peekable();
    default_parse(&mut iterator)
}
