use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    SPECIAL_CHARACTER,
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    //literal
    IDENTIFIER,
    STRING(String),
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}
impl Token {
    pub fn get_keywords() -> HashMap<String, Token> {
        return HashMap::from([
            (String::from("and"), Token::AND),
            (String::from("class"), Token::CLASS),
            (String::from("else"), Token::ELSE),
            (String::from("false"), Token::FALSE),
            (String::from("for"), Token::FOR),
            (String::from("if"), Token::IF),
            (String::from("nil"), Token::NIL),
            (String::from("or"), Token::OR),
            (String::from("print"), Token::PRINT),
            (String::from("return"), Token::RETURN),
            (String::from("super"), Token::SUPER),
            (String::from("this"), Token::THIS),
            (String::from("true"), Token::TRUE),
            (String::from("var"), Token::VAR),
            (String::from("while"), Token::WHILE),
        ]);
    }
}
