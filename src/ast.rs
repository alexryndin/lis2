#[derive(Debug)]
enum ErrorKind {
    ErrorGeneral(&'static str),
    ErrorEval(&'static str),
    ErrorUnknSym(&'static str),
}

#[derive(Debug)]
pub struct ASTError {
    error: ErrorKind,
}

#[derive(Debug)]
pub struct Number {
    pub val: i128,
}

impl<'a> Number {
    fn eval(self) -> Result<ValType<'a>, ASTError> {
        Ok(ValType::Number(self))
    }
}

#[derive(Debug)]
pub struct Sexpr<'a> {
    pub val: Vec<ValType<'a>>,
}

impl<'a> Sexpr<'a> {
    fn eval(mut self) -> Result<ValType<'a>, ASTError> {
        self.val = {
            let v: Result<Vec<ValType<'a>>, ASTError> =
                self.val.into_iter().map(ValType::eval).collect();
            v?
        };
        //        for v in self.val.iter_mut() {
        //            // Memove??
        //            *v = v.eval()?;
        //        }

        match self.val.len() {
            0 => Ok(ValType::Nil),
            1 => Ok(self.val.remove(0)),
            _ => match self.val.remove(0) {
                ValType::Symbol(sym) => self.eval_sym(sym.val),
                _ => Err(ASTError {
                    error: ErrorKind::ErrorEval("Not a symbol!"),
                }),
            },
        }
    }

    fn eval_sym(self, sym: &str) -> Result<ValType<'a>, ASTError> {
        match sym {
            "+" => self.eval_add(),
            "-" => self.eval_sub(),
            "*" => self.eval_mul(),
            "/" => self.eval_div(),
            _ => Err(ASTError {
                error: ErrorKind::ErrorUnknSym("Unknown Symbol"),
            }),
        }
    }

    fn eval_add(self) -> Result<ValType<'a>, ASTError> {
        let mut res = 0;
        for i in self.val {
            match i {
                ValType::Number(v) => res += v.val,
                _ => {
                    return Err(ASTError {
                        error: ErrorKind::ErrorEval("NaN"),
                    })
                }
            }
        }
        Ok(ValType::Number(Number { val: res }))
    }
    fn eval_sub(self) -> Result<ValType<'a>, ASTError> {
        let mut res = 0;
        for i in self.val {
            match i {
                ValType::Number(v) => res -= v.val,
                _ => {
                    return Err(ASTError {
                        error: ErrorKind::ErrorEval("NaN"),
                    })
                }
            }
        }
        Ok(ValType::Number(Number { val: res }))
    }
    fn eval_mul(self) -> Result<ValType<'a>, ASTError> {
        let mut res = 1;
        for i in self.val {
            match i {
                ValType::Number(v) => res *= v.val,
                _ => {
                    return Err(ASTError {
                        error: ErrorKind::ErrorEval("NaN"),
                    })
                }
            }
        }
        Ok(ValType::Number(Number { val: res }))
    }
    fn eval_div(self) -> Result<ValType<'a>, ASTError> {
        let mut res = 0;
        for i in self.val {
            match i {
                ValType::Number(v) => res += v.val,
                _ => {
                    return Err(ASTError {
                        error: ErrorKind::ErrorEval("NaN"),
                    })
                }
            }
        }
        Ok(ValType::Number(Number { val: res }))
    }
}

#[derive(Debug)]
pub struct Qexpr<'a> {
    pub val: Sexpr<'a>,
}

impl<'a> Qexpr<'a> {
    fn eval(self) -> Result<ValType<'a>, ASTError> {
        self.val.eval()
    }
}

#[derive(Debug)]
pub struct Symbol<'a> {
    pub val: &'a str,
}

impl<'a> Symbol<'a> {
    fn eval(self) -> Result<ValType<'a>, ASTError> {
        Ok(ValType::Symbol(self))
    }
}

#[derive(Debug)]
pub enum ValType<'a> {
    Number(Number),
    Sexpr(Sexpr<'a>),
    Qexpr(Qexpr<'a>),
    Symbol(Symbol<'a>),
    Nil,
}

impl<'a> ValType<'a> {
    fn eval(self) -> Result<ValType<'a>, ASTError> {
        match self {
            ValType::Number(v) => v.eval(),
            ValType::Sexpr(v) => v.eval(),
            ValType::Qexpr(v) => v.eval(),
            ValType::Symbol(v) => v.eval(),
            ValType::Nil => Ok(ValType::Nil),
        }
    }
}

#[derive(Debug)]
pub struct AST<'a> {
    pub a_type: ValType<'a>,
}

impl<'a> AST<'a> {
    pub fn eval(self) -> Result<ValType<'a>, ASTError> {
        self.a_type.eval()
    }
}
