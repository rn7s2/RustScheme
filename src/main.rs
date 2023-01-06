mod data;

use data::{format_sexpr, Atomic, Cons, Number, Sexpr, Symbol};

fn main() {
    let x = Sexpr::Cons(Cons {
        car: Box::new(Sexpr::Atomic(Atomic::Symbol(Symbol("define".to_owned())))),
        cdr: Box::new(Sexpr::Cons(Cons {
            car: Box::new(Sexpr::Atomic(Atomic::Symbol(Symbol("x".to_owned())))),
            cdr: Box::new(Sexpr::Cons(Cons {
                car: Box::new(Sexpr::Atomic(Atomic::Number(Number::Integer(5)))),
                cdr: Box::new(Sexpr::Atomic(Atomic::Null)),
            })),
        })),
    });

    println!("{}", format_sexpr(&x));
}
