mod ast;

use ast::{format_sexpr, Atomic, Sexpr, Symbol, Value};

fn main() {
    let mut x = Sexpr::new();
    x.push(Value::Atomic(Atomic::Symbol(Symbol("define".to_owned()))));
    x.push(Value::Atomic(Atomic::Symbol(Symbol("x".to_owned()))));
    x.push(Value::Sexpr(Box::new(vec![Value::Atomic(Atomic::Symbol(
        Symbol("read".to_owned()),
    ))])));

    println!("{}", format_sexpr(&x));
}
