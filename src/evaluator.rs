use std::collections::HashMap;

use crate::{
    analyzer::{analyze, Procedure, Value},
    lexer::{Cons, Symbol},
    parser::Expression,
};

pub type Env = Vec<HashMap<Symbol, Value>>;
pub const THE_EMPTY_ENV: Env = Vec::new();

pub fn eval(exp: Expression, env: Env) -> Result<Value, ()> {
    analyze(exp).call_once((env,))
}

pub fn apply(proc: Procedure, args: Cons) -> Result<Value, ()> {
    todo!()
}

pub fn extend_env(vals: Vec<(Symbol, Value)>, env: &Env) -> Env {
    let mut hash = HashMap::new();
    for (var, val) in vals {
        hash.insert(var, val);
    }
    //env.append(&mut vec![hash]);
    vec![vec![hash], env.clone()].concat()
}

pub fn primitive_procs() -> Vec<(Symbol, Value)>
