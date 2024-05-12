mod util;

use std::{fs, io};

use crate::util::scanner::Scanner;

pub fn run_file(file_path: &str) {
    let file_contents = String::from_utf8(fs::read(file_path).unwrap()).unwrap();

    run(&file_contents);
}
pub fn run_prompt() {
    println!("Starting prompt:");
    loop {
        let mut buffer = String::from("");
        let _ = io::stdin().read_line(&mut buffer).unwrap();

        if buffer == "exit()\n" {
            break;
        }
        run(&buffer);
    }
}
pub fn run(source: &str) {
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
