use std::io;
use std::io::{stdout, Write};


mod token;
mod lexer;
mod ast;
mod parser;

const PROMPT: &str = ">> ";

fn main() {
    println!("Hello {}! This is the Monkey programming language!", whoami::username());
    println!("Feel free to type in commands");
    loop {
        print!("{}", PROMPT);
        stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut lexer = lexer::Lexer::new(buffer);
        loop {
            let token = lexer.next_token();
            if token.token_type == token::TokenType::EOF {
                break;
            }
            println!("{:?}", token);
        }
    }
}
