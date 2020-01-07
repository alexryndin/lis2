// fn repl(prompt: "λ > ") {
use std::io;
use std::io::Write;
use std::rc::Rc;
use crate::parser::Parser;
use crate::env::Env;

pub fn repl(prompt: &str) {
    let env = Env::new(None);
    let env = Rc::new(env);
    loop {
        let env = Rc::clone(&env);
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}", input);
        let mut parser = Parser::new(&input);
        let out = parser.parse().unwrap();
        let out = out.eval(env);


        println!("{:?}", out);
    }
}
