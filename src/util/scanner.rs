use crate::{error, util::token::Token};
use std::iter;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    line: u32,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        return Scanner { source, line: 1 };
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let binding = self.source.clone();
        let mut bytes = binding.as_bytes().iter().peekable();
        let mut c = bytes.next();
        let mut i = 0;

        while c != None {
            let token = self.scan_token(&c.unwrap(), &mut bytes, &mut i);
            match token {
                Ok(ok) => {
                    if let Token::SPECIAL_CHARACTER = ok {
                    } else {
                        tokens.push(ok);
                    }
                }
                Err(e) => {
                    error(self.line, e);
                }
            }

            c = bytes.next();
            i += 1;
        }

        tokens.push(Token::EOF);
        return tokens;
    }
    fn scan_token<'a, I: Iterator<Item = &'a u8>>(
        &self,
        c: &u8,
        iter: &mut iter::Peekable<I>,
        current: &mut usize,
    ) -> Result<Token, &str> {
        match c {
            b'(' => Ok(Token::LEFT_PAREN),
            b')' => Ok(Token::RIGHT_PAREN),
            b'{' => Ok(Token::LEFT_BRACE),
            b'}' => Ok(Token::RIGHT_BRACE),
            b',' => Ok(Token::COMMA),
            b'.' => Ok(Token::DOT),
            b'-' => Ok(Token::MINUS),
            b'+' => Ok(Token::PLUS),
            b';' => Ok(Token::SEMICOLON),
            b'*' => Ok(Token::STAR),
            b'!' => Ok(if self.is_expected(b'=', iter) {
                Token::BANG_EQUAL
            } else {
                Token::BANG
            }),
            b'=' => Ok(if self.is_expected(b'=', iter) {
                Token::EQUAL_EQUAL
            } else {
                Token::EQUAL
            }),
            b'<' => Ok(if self.is_expected(b'=', iter) {
                Token::LESS_EQUAL
            } else {
                Token::LESS
            }),
            b'>' => Ok(if self.is_expected(b'=', iter) {
                Token::GREATER_EQUAL
            } else {
                Token::GREATER
            }),
            b'/' => {
                if self.is_expected(b'/', iter) {
                    while iter.peek() != None && **iter.peek().expect("idk") != b'\n' {
                        iter.next();
                        *current += 1;
                    }
                    iter.next();
                    *current += 1;
                    return Ok(Token::SPECIAL_CHARACTER);
                } else {
                    Ok(Token::SLASH)
                }
            }
            b' ' => Ok(Token::SPECIAL_CHARACTER),
            b'\r' => Ok(Token::SPECIAL_CHARACTER),
            b'\t' => Ok(Token::SPECIAL_CHARACTER),
            b'\n' => Ok(Token::SPECIAL_CHARACTER),
            b'\"' => self.get_string(iter, current),
            c => {
                if c.is_ascii_digit() {
                    return self.get_number(iter, current);
                }
                Err("Unexpected character")
            }
        }
    }
    fn get_string<'a, I: Iterator<Item = &'a u8>>(
        &self,
        iter: &mut iter::Peekable<I>,
        current: &mut usize,
    ) -> Result<Token, &str> {
        *current += 1;
        let start = *current;

        let mut buffer = iter.next();
        if buffer == None {
            return Err("Unterminated string");
        }

        while *buffer.unwrap() != b'\"' {
            *current += 1;

            buffer = iter.next();
            if buffer == None {
                return Err("Unterminated string");
            }
        }
        let a = String::from_utf8(self.source.as_bytes()[start..*current].to_vec());
        return match a {
            Ok(ok) => Ok(Token::STRING(ok)),
            Err(_) => Err("Invalid UTF-8 characters"),
        };
    }
    fn get_number<'a, I: Iterator<Item = &'a u8>>(
        &self,
        iter: &mut iter::Peekable<I>,
        current: &mut usize,
    ) -> Result<Token, &str> {
        let start = *current;
        while iter.peek().unwrap().is_ascii_digit() {
            iter.next();
            *current += 1;
        }
        if *iter.next().unwrap() == b'.' {
            if iter.peek().unwrap().is_ascii_digit() {
                while iter.peek().unwrap().is_ascii_digit() {
                    iter.next();
                    *current += 1;
                }
                let a = String::from_utf8(self.source.as_bytes()[start..*current + 1].to_vec());
                let a = match a {
                    Ok(ok) => match ok.parse::<f64>() {
                        Ok(k) => Ok(k),
                        Err(_) => Err("Could parse string to number"),
                    },
                    Err(_) => Err("Invalid UTF-8 characters"),
                };
                return match a {
                    Ok(ok) => Ok(Token::NUMBER(ok)),
                    Err(e) => Err(e),
                };
            } else {
                return Err("Did not find digit after '.'");
            }
        }
        let a = String::from_utf8(self.source.as_bytes()[start..*current + 1].to_vec());
        let a = match a {
            Ok(ok) => match ok.parse::<f64>() {
                Ok(k) => Ok(k),
                Err(_) => Err("Could parse string to number"),
            },
            Err(_) => Err("Invalid UTF-8 characters"),
        };
        return match a {
            Ok(ok) => Ok(Token::NUMBER(ok)),
            Err(e) => Err(e),
        };
    }
    fn is_expected<'a, I: Iterator<Item = &'a u8>>(
        &self,
        expect: u8,
        iter: &mut iter::Peekable<I>,
    ) -> bool {
        if **iter.peek().expect("idk bro") != expect {
            return false;
        }
        iter.next();
        return true;
    }
}
