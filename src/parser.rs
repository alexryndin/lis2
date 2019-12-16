use crate::ast::{Number, Qexpr, Sexpr, Symbol, ValType, Val, AST};
use std::rc::Rc;
use crate::token::{Token, Tokenizer2};
use std::iter::Iterator;

#[derive(Debug)]
enum ErrorKind {
//    TokenizerError,
    ParserError,
    ParseSexprError,
    IntegerParseError,
    ExprParseError,
}

#[derive(Debug)]
pub struct ParserError {
    error: ErrorKind,
}

pub struct Parser<'a> {
    t: std::iter::Peekable<Tokenizer2<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let t: Tokenizer2<'a> = Tokenizer2::new(input);

        Parser {
            t: t.peekable(),
        }
    }

    fn parse_sexpr(&mut self) -> Result<Sexpr, ParserError> {
        // Pass lparen
        self.t.next();
        let mut ret: Vec<Val> = Vec::new();
        loop {
            let token = self.t.peek();
            match token {
                Some(token2) => match token2 {
                    Ok(Token::RParen) => {
                        self.t.next();
                        return Ok(Sexpr::new(ret));
                    }
                    _ => {
                        let val = self.parse_expr()?;
                        ret.push(Rc::new(val));
                    }
                },
                None => {
                    return Err(ParserError {
                        error: ErrorKind::ParseSexprError,
                    });
                }
            }
        }
    }
    //Err(ErrorKind::ParseSexprError)

    fn parse_qexpr(&mut self) -> Result<Qexpr, ParserError> {
        self.t.next();
        Ok(Qexpr::new(self.parse_sexpr()?))
    }

    fn parse_integer(&mut self) -> Result<Number, ParserError> {
        let sym = self.t.next().unwrap().unwrap();
        match sym {
            Token::Number(num) => {
                let num = num.parse();
                match num {
                    Ok(num) => Ok(Number::new(num)),
                    Err(_) => Err(ParserError {
                        error: ErrorKind::IntegerParseError,
                    }),
                }
            }
            _ => Err(ParserError {
                error: ErrorKind::IntegerParseError,
            }),
        }
    }

    fn parse_symbol(&mut self) -> Result<Symbol, ParserError> {
        if let Token::Symbol(v) = self.t.next().unwrap().unwrap() {
            Ok(Symbol::new(v.to_owned()))
        } else {
            panic!("Something gone wrong!");
        }
    }

    pub fn parse(&mut self) -> Result<AST, ParserError> {
        Ok(AST::new(Rc::new(self.parse_expr()?)))
    }

    pub fn parse_expr(&mut self) -> Result<ValType, ParserError> {
        if let Some(token) = self.t.peek() {
            match token {
                Ok(Token::LParen) => Ok(ValType::Sexpr(self.parse_sexpr()?)),
                Ok(Token::Quote) => Ok(ValType::Qexpr(self.parse_qexpr()?)),
                Ok(Token::Number(_)) => Ok(ValType::Number(self.parse_integer()?)),
                Ok(Token::Symbol(_)) => Ok(ValType::Symbol(self.parse_symbol()?)),
                _ => Err(ParserError {
                    error: ErrorKind::ExprParseError,
                }),
            }
        } else {
            Err(ParserError {
                error: ErrorKind::ParserError,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_simple_ast_works() {
        let input = String::from("(+ 2 2)");
        let out = Parser::new(&input).parse().unwrap().eval().unwrap();

        assert_eq!(out, ValType::Number(Number::new(4)));
    }

}
