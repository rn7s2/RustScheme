use crate::lexer::{Atomic, Number, Sexpr, Symbol};

impl Sexpr {
    fn is_tagged_list(&self, tag: Symbol) -> bool {
        match &self {
            Sexpr::Cons(c) => match &*c.car {
                Sexpr::Atomic(a) => match a {
                    Atomic::Symbol(s) => *s == tag,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    fn is_self_evalutaing(&self) -> bool {
        match &self {
            Sexpr::Atomic(a) => match a {
                Atomic::Number(_) | Atomic::String(_) => true,
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
            Sexpr::Atomic(a) => match a {
                Atomic::Symbol(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

pub enum SelfEval {
    Number(Number),
    String(String),
}

pub enum Expression {
    SelfEval(SelfEval),
    Quoted,
    Variable,
    Assignment,
    Definition,
    If,
    Lambda,
    Begin,
    Cond,
    Application,
}

pub fn parse(sexpr: Sexpr) -> Expression {
    todo!()
}
