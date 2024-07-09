use std::io::{self, Write};

mod ast;
mod lexer;
mod parser;
mod utils;

use parser::Parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim();

        if input == "quit" || input == "exit" {
            break;
        }

        let output = Parser::parse(&input.to_string());
        println!("{:?}", output);
        continue;
    }
}
