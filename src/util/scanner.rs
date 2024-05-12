use crate::util::token::Token;

#[derive(Debug)]
pub struct Scanner {
    source: String,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        return Scanner { source };
    }
    pub fn scan_tokens(&self) -> Vec<Token> {
        let tokens: Vec<Token> = vec![Token::STRING(String::from("print")), Token::NUMBER(34)];
        println!("{}", self.source);
        return tokens;
    }
}
