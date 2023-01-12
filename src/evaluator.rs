use std::collections::HashMap;

use crate::{
    analyzer::{analyze, Procedure, Value},
    lexer::{Atom, Cons, Symbol},
    parser::Expression,
};

pub type Frame = HashMap<Symbol, Value>;
pub type Env = Vec<Frame>;
pub const THE_EMPTY_ENV: Env = Vec::new();

pub fn eval(exp: Expression, env: &mut Env) -> Result<Value, ()> {
    let (proc, param) = analyze(exp).unwrap();
    proc(env, param)
}

pub fn apply(proc: Procedure, args: Cons) -> Result<Value, ()> {
    match proc {
        Procedure::Primitive(p) => match p.as_str() {
            "" => Ok(Value::Atom(Atom::Null)),
            _ => Err(()),
        },
        Procedure::Compound(p) => {
            // make a p.parameters -> args HashMap
            // and call extend_env(, env) to generate a Env
            // then applys accordingly.
        }
    }
}

pub fn define_variable(var: Symbol, val: Value, env: &mut Env) {
    let frame = &mut env[0];
    frame.insert(var, val);
}

pub fn extend_env(vals: Vec<(Symbol, Value)>, env: &Env) -> Env {
    let mut hash = HashMap::new();
    for (var, val) in vals {
        hash.insert(var, val);
    }
    vec![vec![hash], env.clone()].concat()
}

pub fn primitive_procs() -> Vec<(Symbol, Value)> {
    vec![(
        Symbol("pair?".to_owned()),
        Value::Procedure(Procedure::Primitive("pair?".to_owned())),
        // ... add more primitive procedures
    )]
}
