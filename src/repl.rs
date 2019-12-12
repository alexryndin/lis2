// fn repl(prompt: "λ > ") {
use std::io;
use std::io::Write;
use crate::parser::Parser;

pub fn repl(prompt: &str) {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}", input);
        let mut parser = Parser::new(&input);
        let out = parser.parse().unwrap();
        let out = out.eval();


        println!("{:?}", out);
    }
}
