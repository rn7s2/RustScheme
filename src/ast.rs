pub enum Number {
    Integer(i32),
    Float(f64),
}

pub struct Symbol(pub String);

pub enum Atomic {
    Number(Number),
    String(String),
    Symbol(Symbol),
}

pub enum Value {
    Atomic(Atomic),
    Sexpr(Box<Sexpr>),
}

pub type Sexpr = Vec<Value>;

pub fn format_value(value: &Value) -> String {
    match value {
        Value::Atomic(v) => match v {
            Atomic::Number(n) => match n {
                Number::Integer(v) => format!("{}", v),
                Number::Float(v) => format!("{}", v),
            },
            Atomic::String(s) => format!("{}", s),
            Atomic::Symbol(s) => format!("{}", s.0),
        },
        Value::Sexpr(bs) => format_sexpr(bs),
    }
}

pub fn format_sexpr(sexpr: &Sexpr) -> String {
    format!(
        "({})",
        sexpr.iter().fold(String::new(), |result, x| {
            if result.is_empty() {
                format_value(x)
            } else {
                format!("{} {}", result, format_value(x))
            }
        })
    )
}
