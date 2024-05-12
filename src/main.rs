use lox_rust::{run_file, run_prompt};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Incorrect usage, it should be rlox -- (file)");
        process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
