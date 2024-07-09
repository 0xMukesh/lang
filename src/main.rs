use std::{
    io::{self, Write},
    process::exit,
};

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod utils;

use parser::Parser;

use crate::interpreter::{Interpreter, RuntimeValues};

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

        for expr in output.body {
            let eval = Interpreter::evaluate(expr);

            if let RuntimeValues::Error(err) = eval {
                eprintln!("{}", err);
                exit(1);
            } else {
                println!("{:?}", eval)
            }
        }

        continue;
    }
}
