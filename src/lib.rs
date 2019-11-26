use std::str;

#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    Literal(String),
    Quote,
    Dot,
}

pub struct Tokenizer2<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer2<'a> {
    pub fn new(input: &str) -> Tokenizer2 {
        Tokenizer2 {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn get(&mut self) -> Option<u8> {
        if self.pos < self.input.len() {
            self.pos += 1;
            Some(self.input[self.pos - 1])
        } else {
            None
        }
    }

    fn put(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    fn collect(&mut self) -> Result<&'a [u8], ()> {
        let (start, mut end): (usize, usize) = (self.pos-1, self.pos);
        while let Some(v) = self.get() {
            match v as char {
                'a'..='z' | '+' | '0'..='9' => end += 1,
                _ => {
                    self.put();
                    break;
                }
            }
        }
        //TODO: return also Errors
     //   println!("{}", )
        Ok(&self.input[start..end])
    }

    fn collect_number(&mut self) -> Result<&'a [u8], ()> {
        let (start, mut end): (usize, usize) = (self.pos - 1, self.pos);
        while let Some(v) = self.get() {
            match v as char {
                '0'..='9' | '.' => end += 1,
                _ => {
                    self.put();
                    break;
                }
            }
        }
        //TODO: return also Errors
        Ok(&self.input[start..end])
    }

    pub fn next(&mut self) -> Option<Token> {
        if let Some(t) = self.get() {
            match t as char {
                ' ' => self.next(),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                'a'..='z' | '+' | '-' | '*' | '/' => {
                    if let Ok(v) = self.collect() {
                        Some(Token::Symbol(str::from_utf8(v).unwrap().to_owned()))
                    } else {
                        None
                    }
                }
                '0'..='9' => {
                    if let Ok(v) = self.collect_number() {
                        Some(Token::Symbol(str::from_utf8(v).unwrap().to_owned()))
                    } else {
                        None
                    }
                }
                '.' => Some(Token::Dot),
                '\'' => Some(Token::Quote),
                _ => Some(Token::Quote),
            }
        } else {
            None
        }
    }
}

pub struct Tokenizer<'a> {
    iter: std::slice::Iter<'a, u8>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            iter: input.as_bytes().iter(),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if let Some(t) = self.iter.next() {
            match *t as char {
                '(' => Some(Token::LParen),
                _ => Some(Token::Quote),
            }
        } else {
            None
        }
    }
}
