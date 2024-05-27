mod util;

use std::{fs, io};

use util::{parser::Parser, token::Token};

use crate::util::scanner::Scanner;

pub fn run_file(file_path: &str) {
    let file_contents = String::from_utf8(fs::read(file_path).unwrap()).unwrap();

    run(file_contents);
}
pub fn run_prompt() {
    println!("Starting prompt:");
    loop {
        let mut buffer = String::from("");
        let _ = io::stdin().read_line(&mut buffer).unwrap();

        if buffer == "exit()\n" {
            break;
        }
        run(buffer);
    }
}
pub fn run(source: String) {
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens);
}
pub fn error(line: u32, message: &str) {
    eprintln!("[line {line}] Error: {message}");
}
pub fn parse_error(token: Token, line: u32, message: &str) {
    error(line, format!("{:?}, {}", token, message).as_str());
}
