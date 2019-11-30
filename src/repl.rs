// fn repl(prompt: "λ > ") {
use crate::token;
use std::io;
use std::io::Write;

pub fn repl(prompt: &str) {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut t = token::Tokenizer2::new(&input);
        loop {
            match t.next() {
                Ok(v) => match v {
                    token::Token::EOF => {
                        println!("{:?}", v);
                        break;
                    }
                    _ => {
                        print!("{:?} ", v);
                        io::stdout().flush().unwrap();
                    }
                },
                Err(v) => {
                    println!("{:?} ", v);
                    break;
                }
            }
        }
    }
}
