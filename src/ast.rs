use crate::env::Env;
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

pub struct Func {
    fun: fn(Sexpr, &mut Env) -> Result<Val, ASTError>,
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        self.fun as usize == other.fun as usize
    }
}

impl fmt::Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Fun <{}>", self.fun as usize)
    }
}


pub struct Closure {
    fun: Box<dyn Fn(Sexpr, &mut Env) -> Result<Val, ASTError>>,
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

#[derive(PartialEq, Debug)]
pub enum FuncType {
    Function(Func),
    Closure(Closure),
}

#[derive(PartialEq, Debug)]
pub struct Function {
    fun: FuncType,
}

// impl fmt::Debug for Function {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self.fun {
//             FuncType::Function(f) => write!(f, "Fun <{}>", f.val as usize),
//             FuncType::Closure(_) => write!(f, "Closure <>"),
//         }
//     }
// }

impl Function {
//    pub fn new(fun: fn(Val, &Env) -> Result<Val, ASTError>) -> Function {
//        Function { fun }
//    }

    pub fn new_function(fun: fn(Sexpr, &mut Env) -> Result<Val, ASTError>) -> Val {
        Rc::new(ValType::Function(Function { fun: FuncType::Function ( Func{ fun })}))
    }

    pub fn new_closure(fun: Box<dyn Fn(Sexpr, &mut Env) -> Result<Val, ASTError>>, sym: &str) -> Val {
        Rc::new(ValType::Function(Function { fun: FuncType::Closure( Closure{ fun: fun, sym: sym.to_owned() })}))
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
        Ok(Rc::new(ValType::Number(Number{ val: self.val})))
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

    fn eval(&self, env: &mut Env) -> Result<Val, ASTError> {
        let mut val = {
            let v: Result<Vec<Val>, ASTError> = self
                .val
                .iter()
                .map(|x| ValType::eval(&*x, env))
                .collect();
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
                ValType::Function(fun) => {
                    match &fun.fun {
                        FuncType::Function(fun) => (fun.fun)(Sexpr::new(val), env),
                        FuncType::Closure(fun) => (fun.fun)(Sexpr::new(val), env),
                    }
                }
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
        Qexpr { val: Rc::new(ValType::Sexpr(val)) }
    }

    fn eval(&self, _: &mut Env) -> Result<Val, ASTError> {
        Ok(Rc::clone(&self.val))
    }
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    pub val: String,
}

impl Symbol {
    pub fn new(val: String) -> Symbol {
        Symbol { val }
    }

    fn eval(&self, env: &Env) -> Result<Val, ASTError> {
        match env.get(&self.val) {
            Some(v) => Ok(v),
            None => Err(ASTError {
                error: ErrorKind::ErrorEval("Sym not found!"),
            }),
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
    Function(Function),
    Nil,
}

pub type Val = Rc<ValType>;

impl ValType {
    fn eval(&self, env: &mut Env) -> Result<Val, ASTError> {
        match &self {
            ValType::Number(v) => v.eval(),
            ValType::Sexpr(v) => v.eval(env),
            ValType::Qexpr(v) => v.eval(env),
            ValType::Symbol(v) => v.eval(env),
            ValType::Nil => Ok(Rc::new(ValType::Nil)),
            ValType::Function(_) => Err(ASTError {
                error: ErrorKind::ErrorEval("Function tried to evaluate -- this should not have happened"),
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
    pub fn eval(&self, env: &mut Env) -> Result<Val, ASTError> {
        self.a_type.eval(env)
    }
}
