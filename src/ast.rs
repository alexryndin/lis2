use crate::env::ParentEnv;
use crate::env::{Env, EnvRef, EnvError};
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum ErrorKind {
    ErrorGeneral(&'static str),
    ErrorEval(&'static str),
    ErrorUnknSym(&'static str),
}

#[derive(Debug)]
pub struct ASTError {
    pub error: ErrorKind,
}

pub struct Function {
    fun: fn(Sexpr, EnvRef) -> Result<Val, ASTError>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.fun as usize == other.fun as usize
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fun <{}>", self.fun as usize)
    }
}

pub struct Closure {
    fun: Box<dyn Fn(Sexpr, EnvRef) -> Result<Val, ASTError>>,
    sym: String,
}

impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl fmt::Debug for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure <>")
    }
}

#[derive(PartialEq)]
pub struct Lambda {
    params: Vec<Symbol>,
    body: Val,
    env: Option<Env>,
}

impl Lambda {

    fn new_partial(body: Val, params: Vec<Symbol>, env: Env)  -> Result<Val, ASTError> {
        Ok(Rc::new(ValType::Function(FuncType::Lambda(Lambda {
            params,
            body,
            env: Some(env),
        }))))
    }
  //  pub fn new_val(body: Val, params: Val, env: EnvRef) -> Result<Val, ASTError> {
    pub fn new_val(body: Val, params: Val) -> Result<Val, ASTError> {
        let body = match &*body {
            ValType::Sexpr(_) => body,
            _ => {
                return Err(ASTError {
                    error: ErrorKind::ErrorEval("setq -- expected Sexpr as a first arg"),
                })
            }
        };

        let params = match &*params {
            ValType::Sexpr(v) => v,
            _ => {
                return Err(ASTError {
                    error: ErrorKind::ErrorEval("setq -- expected Sexpr as a first arg"),
                })
            }
        };
        let params = {
            let v: Result<Vec<Symbol>, ASTError> = params
                .val
                .iter()
                .map(|x| match &**x {
                    ValType::Symbol(v) => Ok(v.clone()),
                    _ => Err(ASTError {
                        error: ErrorKind::ErrorEval("Lambda passed not a symbol"),
                    }),
                })
                .collect();
            v?
        };

        let env = Env::new(None);

        Ok(Rc::new(ValType::Function(FuncType::Lambda(Lambda {
            params,
            body,
            env: None,
        }))))
    }

    fn call(&self, val: Sexpr, parent_env: EnvRef) -> Result<Val, ASTError> {
//        if val.val.len() > self.params.len() {
//            return Err(ASTError {
//                error: ErrorKind::ErrorEval("lambda eval -- too many args"),
//            });
//        }
        println!("{}", "Lambda called!");
        let mut env = match &self.env {
            None => Env::new(None),
            Some(e) => e.clone(),

        };
        let mut params = self.params.clone();
        for v in val.val.into_iter().rev() {
            let s = match params.pop() {
                None => return Err(ASTError {
                error: ErrorKind::ErrorEval("lambda eval -- too many args"),
            }),
                Some(v) => v.val,
            };
            
            env.put(s, v);
        };


        if params.len() > 0 {
        let body = Val::clone(&self.body);
            Lambda::new_partial(body, params, env)
        } else {
            env = env.set_parent(Some(parent_env)).unwrap();
            self.body.eval(Rc::new(env))
        }


        



    }
}

impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lambda <>")
    }
}

// impl PartialEq for Lambda{
//     fn eq(&self, other: &Self) -> bool {
//         true
//     }
// }

#[derive(PartialEq, Debug)]
pub enum FuncType {
    Function(Function),
    Closure(Closure),
    Lambda(Lambda),
}

impl FuncType {
    //    pub fn new(fun: fn(Val, &Env) -> Result<Val, ASTError>) -> Function {
    //        Function { fun }
    //    }

    pub fn new_function(fun: fn(Sexpr, EnvRef) -> Result<Val, ASTError>) -> Val {
        Rc::new(ValType::Function(FuncType::Function(Function { fun })))
    }

    pub fn new_closure(
        fun: Box<dyn Fn(Sexpr, EnvRef) -> Result<Val, ASTError>>,
        sym: &str,
    ) -> Val {
        Rc::new(ValType::Function(FuncType::Closure(Closure {
            fun: fun,
            sym: sym.to_owned(),
        })))
    }

    // Uncomment to realize that Sexpr::eval is the only good place
    // to eval function (no need to clone or smth)
    //fn eval(self, val: Val, env: &Env) -> Result<Val, ASTError> {
    //    let val = match *val {
    //        ValType::Sexpr(v) => v,
    //        _ => {
    //            return Err(ASTError {
    //                error: ErrorKind::ErrorEval("add expected sexpr"),
    //            })
    //        }
    //    };

    //    (self.fun)(val, env)
    //}
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub val: i128,
}

impl Number {
    pub fn new(val: i128) -> Number {
        Number { val }
    }

    fn eval(&self) -> Result<Val, ASTError> {
        Ok(Rc::new(ValType::Number(Number { val: self.val })))
    }
}

#[derive(Debug, PartialEq)]
pub struct Sexpr {
    pub val: Vec<Val>,
}

impl Sexpr {
    pub fn new(val: Vec<Val>) -> Sexpr {
        Sexpr { val }
    }

    fn eval(&self, env: EnvRef) -> Result<Val, ASTError> {
        let mut val = {
            let v: Result<Vec<Val>, ASTError> =
                self.val.iter().map(|x| ValType::eval(&*x, Rc::clone(&env))).collect();
            v?
        };
        //        for v in self.val.iter_mut() {
        //            // Memove??
        //            *v = v.eval()?;
        //        }

        match self.val.len() {
            0 => Ok(Rc::new(ValType::Nil)),
            1 => Ok(val.remove(0)),
            _ => match &*val.remove(0) {
                ValType::Function(fun) => match fun {
                    FuncType::Function(fun) => (fun.fun)(Sexpr::new(val), env),
                    FuncType::Closure(fun) => (fun.fun)(Sexpr::new(val), env),
                    FuncType::Lambda(fun) => fun.call(Sexpr::new(val), env),
                },
                _ => Err(ASTError {
                    error: ErrorKind::ErrorEval("Not a symbol!"),
                }),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Qexpr {
    val: Val,
}

impl Qexpr {
    pub fn new(val: Sexpr) -> Qexpr {
        Qexpr {
            val: Rc::new(ValType::Sexpr(val)),
        }
    }

    fn eval(&self, _: EnvRef) -> Result<Val, ASTError> {
        Ok(Rc::clone(&self.val))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub val: String,
}

impl Symbol {
    pub fn new(val: String) -> Symbol {
        Symbol { val }
    }

    fn eval(&self, env: EnvRef) -> Result<Val, ASTError> {
        match env.get(&self.val) {
            Some(v) => Ok(v),
            None => { println!("{}", self.val); Err(ASTError {
                error: ErrorKind::ErrorEval("Sym not found!"),
            })},
        }
    }
}

// TODO: Consider to make private
#[derive(Debug, PartialEq)]
pub enum ValType {
    Number(Number),
    Sexpr(Sexpr),
    Qexpr(Qexpr),
    Symbol(Symbol),
    Function(FuncType),
    Nil,
}

pub type Val = Rc<ValType>;

impl ValType {
    fn eval(&self, env: EnvRef) -> Result<Val, ASTError> {
        match &self {
            ValType::Number(v) => v.eval(),
            ValType::Sexpr(v) => v.eval(env),
            ValType::Qexpr(v) => v.eval(env),
            ValType::Symbol(v) => v.eval(env),
            ValType::Nil => Ok(Rc::new(ValType::Nil)),
            ValType::Function(_) => Err(ASTError {
                error: ErrorKind::ErrorEval(
                    "Function tried to evaluate -- this should not have happened",
                ),
            }),
        }
    }
}

#[derive(Debug)]
pub struct AST {
    a_type: Val,
}

impl AST {
    pub fn new(val: Val) -> AST {
        AST { a_type: val }
    }
    pub fn eval(&self, env: EnvRef) -> Result<Val, ASTError> {
        self.a_type.eval(env)
    }
}
