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

#[derive(Debug, PartialEq)]
pub struct Number {
    val: i128,
}

impl Number {
    pub fn new(val: i128) -> Number {
        Number { val }
    }

    fn eval(self) -> Result<ValType, ASTError> {
        Ok(ValType::Number(self))
    }
}

#[derive(Debug, PartialEq)]
pub struct Sexpr {
    val: Vec<ValType>,
}

impl Sexpr {
    pub fn new(val: Vec<ValType>) -> Sexpr {
        Sexpr { val }
    }

    fn eval(mut self) -> Result<ValType, ASTError> {
        self.val = {
            let v: Result<Vec<ValType>, ASTError> =
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

    fn eval_sym(self, sym: String) -> Result<ValType, ASTError> {
        match sym.as_ref() {
            "+" => self.eval_add(),
            "-" => self.eval_sub(),
            "*" => self.eval_mul(),
            "/" => self.eval_div(),
            _ => Err(ASTError {
                error: ErrorKind::ErrorUnknSym("Unknown Symbol"),
            }),
        }
    }

    fn eval_add(self) -> Result<ValType, ASTError> {
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
    fn eval_sub(self) -> Result<ValType, ASTError> {
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
    fn eval_mul(self) -> Result<ValType, ASTError> {
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
    fn eval_div(self) -> Result<ValType, ASTError> {
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

#[derive(Debug, PartialEq)]
pub struct Qexpr {
    val: Sexpr,
}

impl Qexpr {
    pub fn new(val: Sexpr) -> Qexpr {
        Qexpr { val }
    }

    fn eval(self) -> Result<ValType, ASTError> {
        self.val.eval()
    }
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    val: String,
}

impl Symbol {
    pub fn new(val: String) -> Symbol {
        Symbol { val }
    }

    fn eval(self) -> Result<ValType, ASTError> {
        Ok(ValType::Symbol(self))
    }
}

#[derive(Debug, PartialEq)]
pub enum ValType {
    Number(Number),
    Sexpr(Sexpr),
    Qexpr(Qexpr),
    Symbol(Symbol),
    Nil,
}

impl ValType {
    fn eval(self) -> Result<ValType, ASTError> {
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
pub struct AST {
    a_type: ValType,
}

impl AST {
    pub fn new(val: ValType) -> AST {
        AST { a_type: val }
    }
    pub fn eval(self) -> Result<ValType, ASTError> {
        self.a_type.eval()
    }
}

