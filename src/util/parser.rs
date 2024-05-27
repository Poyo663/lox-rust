use core::panic;
use std::cell::RefCell;

use super::{
    expression::{Expr, Expression},
    token::Token,
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser {
            tokens,
            current: RefCell::new(0),
        };
    }
    fn set_current(&self, value: usize) {
        let mut ptr = self.current.borrow_mut();
        *ptr = value;
    }
    fn expression(&self) -> Expr {
        let mut current = *self.current.borrow();
        let exp = self.equality(&mut current);
        self.set_current(current);

        return exp;
    }

    fn equality(&self, current: &mut usize) -> Expr {
        let expected = [Token::BANG_EQUAL, Token::EQUAL_EQUAL];
        let mut expression = self.comparison(current);

        while self.expect(current, &expected) {
            let operator = self.previous(*current);
            let right = self.comparison(current);
            expression = Expression::new_binary(expression, operator.clone(), right);
        }
        return expression;
    }
    fn comparison(&self, current: &mut usize) -> Expr {
        let expected = [
            Token::GREATER,
            Token::GREATER_EQUAL,
            Token::LESS,
            Token::LESS_EQUAL,
        ];
        let mut expression = self.term(current);

        while self.expect(current, &expected) {
            let operator = self.previous(*current);
            let right = self.term(current);
            expression = Expression::new_binary(expression, operator.clone(), right);
        }
        return expression;
    }
    fn term(&self, current: &mut usize) -> Expr {
        let expected = [Token::MINUS, Token::PLUS];
        let mut expression = self.factor(current);

        while self.expect(current, &expected) {
            let operator = self.previous(*current);
            let right = self.factor(current);
            expression = Expression::new_binary(expression, operator.clone(), right);
        }
        return expression;
    }
    fn factor(&self, current: &mut usize) -> Expr {
        let expected = [Token::SLASH, Token::STAR];
        let mut expression = self.unary(current);

        while self.expect(current, &expected) {
            let operator = self.previous(*current);
            let right = self.unary(current);
            expression = Expression::new_binary(expression, operator.clone(), right);
        }
        return expression;
    }
    fn unary(&self, current: &mut usize) -> Expr {
        let expected = [Token::BANG, Token::MINUS];
        if self.expect(current, &expected) {
            let operator = self.previous(*current);
            let right = self.unary(current);
            return Expression::new_unary(operator.clone(), right);
        }
        return self.primary(current);
    }
    fn primary(&self, current: &mut usize) -> Expr {
        if let Token::STRING(_) = self.tokens[*current] {
            self.advance(current);
            return Expression::new_literal(self.tokens[*current].clone());
        }
        match self.tokens[*current] {
            Token::FALSE => {
                self.advance(current);
                return Expression::new_literal(Token::FALSE);
            }
            Token::TRUE => {
                self.advance(current);
                return Expression::new_literal(Token::TRUE);
            }
            Token::NIL => {
                self.advance(current);
                return Expression::new_literal(Token::NIL);
            }
            Token::NUMBER(value) => {
                self.advance(current);
                return Expression::new_literal(Token::NUMBER(value));
            }
            Token::LEFT_PAREN => {
                let expr = self.expression();
                self.consume(current, &Token::RIGHT_PAREN, "Expect ')' after expression");
                return Expression::new_grouping(expr);
            }
            _ => panic!("oh noooooooo"),
        }
    }

    fn expect(&self, current: &mut usize, types: &[Token]) -> bool {
        for tipe in types {
            if self.check(*current, tipe) {
                self.advance(current);
                return true;
            }
        }
        return false;
    }

    fn consume(&self, current: &mut usize, tipe: &Token, message: &str) {
        if self.check(*current, tipe) {
            self.advance(current);
            return;
        }
    }

    fn check(&self, current: usize, tipe: &Token) -> bool {
        if self.is_at_end(current) {
            return false;
        }
        return self.tokens[current] == *tipe;
    }

    fn previous(&self, current: usize) -> &Token {
        return &self.tokens[current - 1];
    }

    fn advance(&self, current: &mut usize) -> &Token {
        if !self.is_at_end(*current) {
            *current += 1;
        };
        return self.previous(*current);
    }

    fn peek(&self, current: usize) -> &Token {
        return &self.tokens[current];
    }

    fn is_at_end(&self, current: usize) -> bool {
        return *self.peek(current) == Token::EOF;
    }
    fn syncronize(&self, current: &mut usize) {
        while !self.is_at_end(*current) {
            if let Token::SEMICOLON = *self.previous(*current) {
                return;
            }

            match *self.peek(*current) {
                Token::CLASS => {
                    return;
                }
                Token::FUN => {
                    return;
                }
                Token::VAR => {
                    return;
                }
                Token::FOR => {
                    return;
                }
                Token::IF => {
                    return;
                }
                Token::WHILE => {
                    return;
                }
                Token::PRINT => {
                    return;
                }
                Token::RETURN => {
                    return;
                }
                _ => {self.advance(current);}
            }
        }
    }
}
