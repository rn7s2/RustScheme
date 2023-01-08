#[derive(Clone, Copy, Debug)]
pub enum Bool {
    True,
    False,
}

#[derive(Clone, Copy, Debug)]
pub enum Number {
    Integer(i32),
    Float(f64),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Symbol(pub String);

#[derive(Clone, Debug)]
pub enum Atom {
    Null,
    Bool(Bool),
    Number(Number),
    String(String),
    Symbol(Symbol),
}

#[derive(Clone, Debug)]
pub struct Cons {
    pub car: Box<Sexpr>,
    pub cdr: Box<Sexpr>,
}

#[derive(Clone, Debug)]
pub enum Sexpr {
    Atom(Atom),
    Cons(Cons),
}

pub fn format_sexpr(value: &Sexpr) -> String {
    match value {
        Sexpr::Atom(v) => match &*v {
            Atom::Null => "()".to_owned(),
            Atom::Bool(b) => match b {
                Bool::True => "#t".to_owned(),
                Bool::False => "#f".to_owned(),
            },
            Atom::Number(n) => match n {
                Number::Integer(v) => format!("{}", v),
                Number::Float(v) => format!("{}", v),
            },
            Atom::String(s) => format!("{}", s),
            Atom::Symbol(s) => format!("{}", s.0),
        },
        Sexpr::Cons(c) => format!("({} . {})", format_sexpr(&*c.car), format_sexpr(&*c.cdr)),
    }
}

pub fn read_sexpr() -> Result<Sexpr, ()> {
    // Ok(Sexpr::Cons(Cons {
    //     car: Box::new(Sexpr::Atom(Atom::Symbol(Symbol("define".to_owned())))),
    //     cdr: Box::new(Sexpr::Cons(Cons {
    //         car: Box::new(Sexpr::Atom(Atom::Symbol(Symbol("x".to_owned())))),
    //         cdr: Box::new(Sexpr::Cons(Cons {
    //             car: Box::new(Sexpr::Atom(Atom::Bool(Bool::False))),
    //             cdr: Box::new(Sexpr::Atom(Atom::Null)),
    //         })),
    //     })),
    // }))

    Ok(Sexpr::Atom(Atom::Null))
}

pub fn lex(text: String) -> Result<Sexpr, String> {
    // for debugging
    // let sexpr = Sexpr::Cons(Cons {
    //     car: Box::new(Sexpr::Atom(Atom::Symbol(Symbol("define".to_owned())))),
    //     cdr: Box::new(Sexpr::Cons(Cons {
    //         car: Box::new(Sexpr::Atom(Atom::Symbol(Symbol("x".to_owned())))),
    //         cdr: Box::new(Sexpr::Cons(Cons {
    //             car: Box::new(Sexpr::Atom(Atom::Bool(Bool::False))),
    //             cdr: Box::new(Sexpr::Atom(Atom::Null)),
    //         })),
    //     })),
    // });

    // let sexpr = Sexpr::Cons(Cons {
    //     car: Box::new(Sexpr::Atom(Atom::Symbol(Symbol("f".to_owned())))),
    //     cdr: Box::new(Sexpr::Cons(Cons {
    //         car: Box::new(Sexpr::Atom(Atom::Number(Number::Integer(5)))),
    //         cdr: Box::new(Sexpr::Atom(Atom::Null)),
    //     })),
    // });

    let sexpr = Sexpr::Atom(Atom::Null);

    println!("{}", format_sexpr(&sexpr));
    Ok(sexpr)
}
