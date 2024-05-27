use super::token::Token;

#[allow(dead_code)]

pub enum Expression {
    Binary(Expr, Token, Expr),
    Grouping(Expr),
    Literal(Token),
    Unary(Token, Expr),
}
pub type Expr = Box<Expression>;

impl Expression {
    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Expr {
        return Box::new(Expression::Binary(left, operator, right));
    }
    pub fn new_grouping(expression: Expr) -> Expr {
        return Box::new(Expression::Grouping(expression));
    }
    pub fn new_literal(value: Token) -> Expr {
        return Box::new(Expression::Literal(value));
    }
    pub fn new_unary(operator: Token, right: Expr) -> Expr {
        return Box::new(Expression::Unary(operator, right));
    }
}
