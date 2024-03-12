use std::io;
use std::io::{stdout, Write};
use std::string::ToString;

mod token;
mod lexer;

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
        while let token = lexer.next_token() {
            if(token.token_type == token::TokenType::EOF) {
                break;
            }
            println!("{:?}", token);
        }
    }
}
