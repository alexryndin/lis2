use std::collections::HashMap;
use crate::ast::{Val, Function};
use crate::builtin;
use std::rc::Rc;

// TODO: Consider different lifetimes for str and parent env
#[derive(Debug ,PartialEq)]
pub struct Env  {
    env: HashMap<String, Val>,
    par: Option<Rc<Env>>,
    
}

impl  Env  {

    fn register_builtins(&mut self) {
        self.put("+".to_owned(), Function::new_closure(builtin::op(0, |a, b| { a + b }), "+"));
        self.put("-".to_owned(), Function::new_closure(builtin::op(0, |a, b| { a - b }), "-"));
        self.put("*".to_owned(), Function::new_closure(builtin::op(1, |a, b| { a * b }), "*"));
        self.put("/".to_owned(), Function::new_closure(builtin::op(1, |a, b| { a / b }), "/"));
        self.put("setq".to_owned(), Function::new_function(builtin::setq));


    }

    pub fn new() -> Env {
        let mut ret = Env {
            env: HashMap::new(),
            par: None
        };
        ret.register_builtins();
        ret
        
    }
    pub fn get(&self, k: &str) -> Option<Val>{
        match self.env.get(k) {
            Some(v) => Some(Rc::clone(v)),
            None => match &self.par {
                Some(v) => v.get(k),
                None => None,
            }
        }
        

    }
    pub fn put(&mut self, k: String, v: Val) {
        self.env.insert(k, v);
        ()
    }

}
