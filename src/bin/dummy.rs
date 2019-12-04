use lis2::repl;
use lis2::ast::Parser;

fn main() {
    let input = "(+ 2 2)";
    let mut parser = Parser::new(input);
    let out = parser.parse_expr();
    println!("{:?}", out);
    repl::repl("λ > ");
}

fn test(v: u8) {
    ()
}
