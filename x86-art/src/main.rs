#[macro_use]
extern crate lazy_static;

use std::{env, path::Path};
mod lexer;
mod runner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let contents = std::fs::read_to_string(Path::new(&args[1]))
            .expect("could not read file")
            .chars()
            .filter(|c| c != &'\r')
            .collect::<String>();
        let tokens = lexer::lex(&contents);
        let parsed = runner::run(&tokens);
    }
}
