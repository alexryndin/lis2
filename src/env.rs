use std::collections::HashMap;
use crate::ast::{Val, FuncType};
use crate::builtin;
use std::rc::Rc;
use std::sync::RwLock;

pub type ParentEnv = Option<Rc<Env>>;

pub type EnvRef = Rc<Env>;

#[derive(Debug)]
pub struct EnvError {
    pub error: &'static str,
}

#[derive(Debug)]
pub struct Env  {
    env: RwLock<HashMap<String, Val>>,
    par: ParentEnv,
}

// TODO: Is this correct implementation?
// More formally: if T: Copy, x: T, and y: &T, then let x = y.clone();
impl Clone for Env {
    fn clone(&self) -> Self {
        let m = self.env.read().unwrap();
        let map = m.clone();
        let par = self.par.clone();
        Env {
            env: RwLock::new(map),
            par
        }
    }
}

// TODO: Implement Eq properly
impl PartialEq for Env{
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl  Env  {

    fn register_builtins(&mut self) {
        self.put("+".to_owned(), FuncType::new_closure(builtin::op(0, |a, b| { a + b }), "+"));
        self.put("-".to_owned(), FuncType::new_closure(builtin::op(0, |a, b| { a - b }), "-"));
        self.put("*".to_owned(), FuncType::new_closure(builtin::op(1, |a, b| { a * b }), "*"));
        self.put("/".to_owned(), FuncType::new_closure(builtin::op(1, |a, b| { a / b }), "/"));
        self.put("setq".to_owned(), FuncType::new_function(builtin::setq));
        self.put("\\".to_owned(), FuncType::new_function(builtin::lambda));


    }

    pub fn set_parent(mut self, par: ParentEnv) -> Result<Env, EnvError> {
        match self.par {
            None => {
                self.par = par;
                Ok(self)
            }
            Some(_) => Err(EnvError {
                error: "Env parent already set"
            })
        }

    }

    pub fn new(par: ParentEnv) -> Env {
        let mut ret = Env {
            env: RwLock::new(HashMap::new()),
            par: par
        };
        ret.register_builtins();
        ret
        
    }
    pub fn get(&self, k: &str) -> Option<Val>{
        let m = self.env.read().unwrap();
        
        match m.get(k) {
            Some(v) => Some(Rc::clone(v)),
            None => match &self.par {
                Some(v) => v.get(k),
                None => None,
            }
        }
        

    }
    pub fn put(&self, k: String, v: Val) {
        let mut m = self.env.write().unwrap();
        m.insert(k, v);
        ()
    }

}
