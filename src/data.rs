pub enum Boolean {
    True,
    False,
}

pub enum Number {
    Integer(i32),
    Float(f64),
}

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

pub enum Value {
    Void,
    Bool(Boolean),
    Number(Number),
    String(String),
    Quote(Box<Sexpr>),
    Lambda,
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
