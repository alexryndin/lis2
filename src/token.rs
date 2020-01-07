use std::str;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    LParen,
    RParen,
    Symbol(&'a str),
    Number(&'a str),
    Literal(&'a str),
    Quote,
    Dot,
    EOF,
}

pub struct Tokenizer2<'a> {
    input: &'a [u8],
    pos: usize,
}

#[derive(Debug, PartialEq)]
enum ErrorKind {
    GeneralError,
}

#[derive(Debug, PartialEq)]
pub struct TokenizerError {
    error: ErrorKind,
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

//    fn collect_identifier(&mut self) -> Result<&'a [u8], ()> {
//        let (start, mut end): (usize, usize) = (self.pos - 1, self.pos);
//        while let Some(v) = self.get() {
//            match v as char {
//                'a'..='z' | '+' | '_' | '-' | '&' | '?' | '!' | '0'..='9' => end += 1,
//                _ => {
//                    self.put();
//                    break;
//                }
//            }
//        }
//        //TODO: return also Errors
//        //   println!("{}", )
//        Ok(&self.input[start..end])
//    }

    fn is_character(v: char) -> bool {
        match v as char {
            'a'..='z' | '+' | '_' | '-' | '&' | '?' | '!' | '0'..='9' => true,
            _ => false,
        }
    }

    fn is_number(v: char) -> bool {
        match v as char {
            '0'..='9' | '.' => true,
            _ => false,
        }
    }

    fn collect(&mut self, pred: fn(char) -> bool) -> Result<&'a [u8], ()> {
        let (start, mut end): (usize, usize) = (self.pos - 1, self.pos);
        while let Some(v) = self.get() {
            match pred(v as char) {
                true => end += 1,
                false => {
                    self.put();
                    break;
                }
            }
        }
        //TODO: return also Errors
        Ok(&self.input[start..end])
    }
}

impl <'a> Iterator for Tokenizer2<'a> {

    type Item = Result<Token<'a>, TokenizerError>;

    fn next(&mut self) -> Option<Result<Token<'a>, TokenizerError>> {
        if let Some(t) = self.get() {
            match t as char {
                ' ' => self.next(),
                '(' => Some(Ok(Token::LParen)),
                ')' => Some(Ok(Token::RParen)),
                'a'..='z' | '+' | '-' | '*' | '/' | '\\' => {
                    if let Ok(v) = self.collect(Self::is_character) {
                        Some(Ok(Token::Symbol(str::from_utf8(v).unwrap())))
                    } else {
                        Some(Err(TokenizerError{error: ErrorKind::GeneralError}))
                    }
                }
                '0'..='9' => {
                    if let Ok(v) = self.collect(Self::is_number) {
                        Some(Ok(Token::Number(str::from_utf8(v).unwrap())))
                    } else {
                        Some(Err(TokenizerError{error: ErrorKind::GeneralError}))
                    }
                }
                '.' => Some(Ok(Token::Dot)),
                '\'' => Some(Ok(Token::Quote)),
                _ => Some(Err(TokenizerError{error: ErrorKind::GeneralError})),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenizer_simple_works() {
        let input = String::from("(+ 2 2)");

        let mut t = Tokenizer2::new(&input);
        assert_eq!(t.next().unwrap().unwrap(), Token::LParen);
        assert_eq!(t.next().unwrap().unwrap(), Token::Symbol("+"));
        assert_eq!(t.next().unwrap().unwrap(), Token::Number("2"));
        assert_eq!(t.next().unwrap().unwrap(), Token::Number("2"));
        assert_eq!(t.next().unwrap().unwrap(), Token::RParen);
        assert_eq!(t.next(), None);
        assert_eq!(t.next(), None);
    }
}
