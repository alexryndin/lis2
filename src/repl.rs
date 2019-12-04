// fn repl(prompt: "λ > ") {
use crate::token;
use std::io;
use std::io::Write;

pub fn repl(prompt: &str) {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}", input);
        let mut t = token::Tokenizer2::new(input.trim());
        loop {
            match t.next() {
                Some(Ok(v)) => match v {
                    token::Token::EOF => {
                        println!("{:?}", v);
                        break;
                    }
                    _ => {
                        print!("{:?} ", v);
                        io::stdout().flush().unwrap();
                    }
                },
                Some(Err(v)) => {
                    println!("{:?} ", v);
                    break;
                },
                None => break,
            }
        }
        println!("");
    }
}
