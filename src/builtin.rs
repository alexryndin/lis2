use crate::ast::{ASTError, ErrorKind, Number, Sexpr, Val, ValType};
use crate::env::Env;
use std::rc::Rc;
//    fn eval_sym(val: ValType) -> Result<ValType, ASTError> {
//        match sym.as_ref() {
//            "+" => self.eval_add(),
//            "-" => self.eval_sub(),
//            "*" => self.eval_mul(),
//            "/" => self.eval_div(),
//            _ => Err(ASTError {
//                error: ErrorKind::ErrorUnknSym("Unknown Symbol"),
//            }),
//        }
//    }

pub fn add(val: Sexpr, _: &mut Env) -> Result<Val, ASTError> {
    let mut res = 0;

    for i in val.val {
        match &*i {
            ValType::Number(v) => res += v.val,
            _ => {
                return Err(ASTError {
                    error: ErrorKind::ErrorEval("NaN"),
                })
            }
        }
    }
    Ok(Rc::new(ValType::Number(Number { val: res })))
}

pub fn op(
    empty: i128,
    _op: fn(i128, i128) -> i128,
) -> Box<dyn Fn(Sexpr, &mut Env) -> Result<Val, ASTError>> {
    Box::new(move |val, _| {
        let mut empty = empty;
        for i in val.val {
            match &*i {
                ValType::Number(v) => empty = _op(empty, v.val),
                _ => {
                    return Err(ASTError {
                        error: ErrorKind::ErrorEval("NaN"),
                    })
                }
            }
        }
        Ok(Rc::new(ValType::Number(Number { val:  empty })))
    })
}

pub fn sub(val: Sexpr, _: &mut Env) -> Result<Val, ASTError> {
    let mut res = 0;

    for i in val.val {
        match &*i {
            ValType::Number(v) => res -= v.val,
            _ => {
                return Err(ASTError {
                    error: ErrorKind::ErrorEval("NaN"),
                })
            }
        }
    }
    Ok(Rc::new(ValType::Number(Number { val: res })))
}
pub fn mul(val: Sexpr, _: &mut Env) -> Result<Val, ASTError> {
    let mut res = 1;

    for i in val.val {
        match &*i {
            ValType::Number(v) => res *= v.val,
            _ => {
                return Err(ASTError {
                    error: ErrorKind::ErrorEval("NaN"),
                })
            }
        }
    }
    Ok(Rc::new(ValType::Number(Number { val: res })))
}
pub fn div(val: Sexpr, _: &mut Env) -> Result<Val, ASTError> {
    let mut res = 0;

    for i in val.val {
        match &*i {
            ValType::Number(v) => res += v.val,
            _ => {
                return Err(ASTError {
                    error: ErrorKind::ErrorEval("NaN"),
                })
            }
        }
    }
    Ok(Rc::new(ValType::Number(Number { val: res })))
}

pub fn setq(val: Sexpr, env: &mut Env) -> Result<Val, ASTError> {
    if val.val.len() < 2 {
        return Err(ASTError {
            error: ErrorKind::ErrorEval("setq -- number of args doesn't match"),
        });
    };

    let vars = match &*val.val[0] {
        ValType::Sexpr(v) => v,
        _ => {
            return Err(ASTError {
                error: ErrorKind::ErrorEval("setq -- expected Sexpr as a first arg"),
            })
        }
    };

    if vars.val.len() != val.val.len() - 1 {
        return Err(ASTError {
            error: ErrorKind::ErrorEval("setq -- number of vars doesn't match"),
        });
    };

    for (i, v) in vars.val.iter().enumerate() {
        match &**v {
            ValType::Symbol(s) => env.put(s.val.to_owned(), Rc::clone(&val.val[i + 1])),
            _ => {
                return Err(ASTError {
                    error: ErrorKind::ErrorEval("setq -- number of vars doesn't match"),
                })
            }
        };
    }

    Ok(Rc::new(ValType::Nil))

    // Checks ^^
}
