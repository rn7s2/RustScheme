pub enum Boolean {
    True,
    False,
}

pub enum Number {
    Integer(i32),
    Float(f64),
}

#[derive(PartialEq)]
pub struct Symbol(pub String);

pub enum Atomic {
    Null,
    Boolean(Boolean),
    Number(Number),
    String(String),
    Symbol(Symbol),
}

pub struct Cons {
    pub car: Box<Sexpr>,
    pub cdr: Box<Sexpr>,
}

pub enum Sexpr {
    Atomic(Atomic),
    Cons(Cons),
}

pub fn format_sexpr(value: &Sexpr) -> String {
    match value {
        Sexpr::Atomic(v) => match v {
            Atomic::Null => "()".to_owned(),
            Atomic::Boolean(b) => match b {
                Boolean::True => "#t".to_owned(),
                Boolean::False => "#f".to_owned(),
            },
            Atomic::Number(n) => match n {
                Number::Integer(v) => format!("{}", v),
                Number::Float(v) => format!("{}", v),
            },
            Atomic::String(s) => format!("{}", s),
            Atomic::Symbol(s) => format!("{}", s.0),
        },
        Sexpr::Cons(c) => format!("({} . {})", format_sexpr(&*c.car), format_sexpr(&*c.cdr)),
    }
}

pub fn lex(text: String) -> Sexpr {
    // for debugging
    Sexpr::Cons(Cons {
        car: Box::new(Sexpr::Atomic(Atomic::Symbol(Symbol("define".to_owned())))),
        cdr: Box::new(Sexpr::Cons(Cons {
            car: Box::new(Sexpr::Atomic(Atomic::Symbol(Symbol("x".to_owned())))),
            cdr: Box::new(Sexpr::Cons(Cons {
                car: Box::new(Sexpr::Atomic(Atomic::Number(Number::Integer(5)))),
                cdr: Box::new(Sexpr::Atomic(Atomic::Null)),
            })),
        })),
    })
}
