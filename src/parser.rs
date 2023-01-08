use crate::lexer::{format_sexpr, Atom, Cons, Number, Sexpr, Symbol};

impl Sexpr {
    fn is_tagged_list(&self, tag: Symbol) -> bool {
        match &self {
            Sexpr::Cons(c) => match &*c.car {
                Sexpr::Atom(a) => match &*a {
                    Atom::Symbol(s) => *s == tag,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    fn is_self_evalutaing(&self) -> bool {
        match &self {
            Sexpr::Atom(a) => match a {
                Atom::Number(_) | Atom::String(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn is_quoted(&self) -> bool {
        self.is_tagged_list(Symbol("quote".to_owned()))
    }

    fn is_variable(&self) -> bool {
        match &self {
            Sexpr::Atom(a) => match a {
                Atom::Symbol(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn is_assignment(&self) -> bool {
        self.is_tagged_list(Symbol("set!".to_owned()))
    }

    fn is_definition(&self) -> bool {
        self.is_tagged_list(Symbol("define".to_owned()))
    }

    fn is_if(&self) -> bool {
        self.is_tagged_list(Symbol("if".to_owned()))
    }

    fn is_lambda(&self) -> bool {
        self.is_tagged_list(Symbol("lambda".to_owned()))
    }

    fn is_begin(&self) -> bool {
        self.is_tagged_list(Symbol("begin".to_owned()))
    }

    fn is_cond(&self) -> bool {
        self.is_tagged_list(Symbol("cond".to_owned()))
    }

    fn is_application(&self) -> bool {
        match &self {
            Sexpr::Cons(_) => true,
            _ => false,
        }
    }

    fn parse_self_evalutaing(self) -> Result<SelfEval, ()> {
        match self {
            Sexpr::Atom(a) => match a {
                Atom::Number(v) => Ok(SelfEval::Number(v)),
                Atom::String(v) => Ok(SelfEval::String(v)),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn parse_quoted(self) -> Result<Sexpr, ()> {
        match self {
            Sexpr::Cons(c) => Ok(*c.cdr),
            _ => Err(()),
        }
    }

    fn parse_variable(self) -> Result<Symbol, ()> {
        match self {
            Sexpr::Atom(a) => match a {
                Atom::Symbol(s) => Ok(s),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn parse_assignment(self) -> Result<(Symbol, Sexpr), ()> {
        match self {
            Sexpr::Cons(c) => match *c.cdr {
                Sexpr::Cons(c) => {
                    let s = match *c.car {
                        Sexpr::Atom(a) => match a {
                            Atom::Symbol(s) => s,
                            _ => return Err(()),
                        },
                        _ => return Err(()),
                    };
                    Ok((s, *c.cdr))
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn parse_definition(self) -> Result<(Symbol, Sexpr), ()> {
        match self {
            Sexpr::Cons(define) => match *define.cdr {
                Sexpr::Cons(c) => match *c.car {
                    // variable
                    Sexpr::Atom(a) => match a {
                        Atom::Symbol(s) => Ok((
                            s,
                            match *c.cdr {
                                Sexpr::Cons(c) => *c.car,
                                _ => return Err(()),
                            },
                        )),
                        _ => Err(()),
                    },
                    // procedure
                    Sexpr::Cons(f_and_params) => match *f_and_params.car {
                        Sexpr::Atom(a) => match a {
                            Atom::Symbol(s) => Ok((
                                s,
                                Sexpr::Cons(Cons {
                                    car: Box::new(Sexpr::Atom(Atom::Symbol(Symbol(
                                        "lambda".to_owned(),
                                    )))),
                                    cdr: Box::new(Sexpr::Cons(Cons {
                                        car: Box::new(*f_and_params.cdr),
                                        cdr: Box::new(*c.cdr),
                                    })),
                                }),
                            )),
                            _ => Err(()),
                        },
                        _ => Err(()),
                    },
                },
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn parse_if(self) -> Result<(Sexpr, Sexpr, Sexpr), ()> {
        match self {
            Sexpr::Cons(c) => match *c.cdr {
                Sexpr::Cons(c) => {
                    let a = *c.car;
                    let (b, c) = match *c.cdr {
                        Sexpr::Cons(c) => (*c.car, *c.cdr),
                        _ => return Err(()),
                    };
                    Ok((a, b, c))
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn parse_lambda(self) -> Result<(Sexpr, Sexpr), ()> {
        match self {
            Sexpr::Cons(c) => match *c.cdr {
                Sexpr::Cons(c) => Ok((*c.car, *c.cdr)),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn parse_begin(self) -> Result<Sexpr, ()> {
        Ok(self)
    }

    fn parse_cond(self) -> Result<Sexpr, ()> {
        Ok(self)
    }

    fn parse_application(self) -> Result<(Sexpr, Sexpr), ()> {
        match self {
            Sexpr::Cons(c) => Ok((*c.car, *c.cdr)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SelfEval {
    Number(Number),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Expression {
    SelfEval(SelfEval),
    Quoted(Sexpr),
    Variable(Symbol),
    Assignment(Symbol, Sexpr),
    Definition(Symbol, Sexpr),
    If(Sexpr, Sexpr, Sexpr),
    Lambda(Sexpr, Sexpr),
    Begin(Sexpr),
    Cond(Sexpr),
    Application(Sexpr, Sexpr),
}

pub fn parse(sexpr: Sexpr) -> Result<Expression, String> {
    if sexpr.is_self_evalutaing() {
        Ok(Expression::SelfEval(sexpr.parse_self_evalutaing().unwrap()))
    } else if sexpr.is_quoted() {
        Ok(Expression::Quoted(sexpr.parse_quoted().unwrap()))
    } else if sexpr.is_variable() {
        Ok(Expression::Variable(sexpr.parse_variable().unwrap()))
    } else if sexpr.is_assignment() {
        let a = sexpr.parse_assignment().unwrap();
        Ok(Expression::Assignment(a.0, a.1))
    } else if sexpr.is_definition() {
        let d = sexpr.parse_definition().unwrap();
        Ok(Expression::Definition(d.0, d.1))
    } else if sexpr.is_if() {
        let i = sexpr.parse_if().unwrap();
        Ok(Expression::If(i.0, i.1, i.2))
    } else if sexpr.is_lambda() {
        let l = sexpr.parse_lambda().unwrap();
        Ok(Expression::Lambda(l.0, l.1))
    } else if sexpr.is_begin() {
        Ok(Expression::Begin(sexpr.parse_begin().unwrap()))
    } else if sexpr.is_cond() {
        Ok(Expression::Cond(sexpr.parse_cond().unwrap()))
    } else if sexpr.is_application() {
        let a = sexpr.parse_application().unwrap();
        Ok(Expression::Application(a.0, a.1))
    } else {
        Err(format!(
            "Unknown expression type -- {}",
            format_sexpr(&sexpr)
        ))
    }
}
