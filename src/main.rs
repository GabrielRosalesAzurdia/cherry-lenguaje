use std::io::{self, BufRead, Write};

pub mod lexer;

fn main() {
    println!("hello, welcome to the cherry programing lenguaje");
    println!("feel free to type comands");
    let stdin = io::stdin();

    loop {
        // Stdout needs to be flushed, due to missing newline
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let mut lexer = lexer::types::new(line);

        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok.type_token == lexer::types::Token::EOF {
                break;
            }
        }
    }
}