use crate::ast::{Number, Qexpr, Sexpr, Symbol, ValType, AST};
use crate::token::{Token, Tokenizer2, TokenizerError};
use std::error::Error;
use std::iter::Iterator;

#[derive(Debug)]
enum ErrorKind {
    TokenizerError,
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
    pos: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let t: Tokenizer2<'a> = Tokenizer2::new(input);

        Parser {
            t: t.peekable(),
            pos: Token::LParen,
        }
    }

    fn parse_sexpr(&mut self) -> Result<Sexpr<'a>, ParserError> {
        // Pass lparen
        self.t.next();
        let mut ret: Vec<ValType<'a>> = Vec::new();
        loop {
            let token = self.t.peek();
            match token {
                Some(token2) => match token2 {
                    Ok(Token::RParen) => {
                        self.t.next();
                        return Ok(Sexpr { val: ret });
                    }
                    _ => {
                        let val = self.parse_expr()?;
                        ret.push(val);
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

    fn parse_qexpr(&mut self) -> Result<Qexpr<'a>, ParserError> {
        self.t.next();
        Ok(Qexpr {
            val: self.parse_sexpr()?,
        })
    }

    fn parse_integer(&mut self) -> Result<Number, ParserError> {
        let sym = self.t.next().unwrap().unwrap();
        match sym {
            Token::Number(num) => {
                let num = num.parse();
                match num {
                    Ok(num) => Ok(Number { val: num }),
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

    fn parse_symbol(&mut self) -> Result<Symbol<'a>, ParserError> {
        if let Token::Symbol(v) = self.t.next().unwrap().unwrap() {
            Ok(Symbol { val: v })
        } else {
            panic!("Something gone wrong!");
        }
    }

    pub fn parse(&mut self) -> Result<AST<'a>, ParserError> {
        Ok(AST {
            a_type: self.parse_expr()?,
        })
    }

    pub fn parse_expr(&mut self) -> Result<ValType<'a>, ParserError> {
        if let Some(token) = self.t.peek() {
            match token {
                Ok(Token::LParen) => Ok(ValType::Sexpr(self.parse_sexpr()?)),
                Ok(Token::Quote) => Ok(ValType::Qexpr(self.parse_qexpr()?)),
                Ok(Token::Number(_)) => Ok(ValType::Number(self.parse_integer()?)),
                Ok(Token::Symbol(v)) => Ok(ValType::Symbol(self.parse_symbol()?)),
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

// impl<'a> Parser<'a> {
//     fn new(input: &'a str) -> Result<Parser<'a>, ParserError> {
//         let mut t: Tokenizer2<'a> = Tokenizer2::new(input);
//                 let token = t.next();
//                 match token {
//                     Err(err) => Err(ParserError{error: ErrorKind::TokenizerError}),
//                     Ok(token) => Ok(Parser {
//                         t: t,
//                         pos: token,
//                     })
//                 }
//
// //        Ok(Parser {
// //            t,
// //            pos: Token::LParen,
// //        })
//     }
// }
