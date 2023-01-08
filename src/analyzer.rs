use crate::{
    evaluator::Env,
    lexer::{Atom, Cons, Sexpr, Symbol},
    parser::{Expression, SelfEval},
};

#[derive(Clone, Debug)]
pub enum Procedure {
    Primitive(String),
    Compound(CompoundProcedure),
}

#[derive(Clone, Debug)]
pub struct CompoundProcedure {
    parameters: Cons,
    body: Cons,
    env: Env,
}

#[derive(Clone, Debug)]
pub enum Value {
    Atom(Atom),
    Cons(Cons),
    Procedure(Procedure),
}

pub fn analyze(exp: Expression) -> Box<dyn FnOnce(&mut Env) -> Result<Value, ()>> {
    match exp {
        Expression::SelfEval(v) => analyze_self_evaluated(v).unwrap(),
        Expression::Quoted(v) => analyze_quoted(v).unwrap(),
        Expression::Variable(v) => analyze_variable(v).unwrap(),
        Expression::Assignment(s, e) => analyze_assignment(s, e).unwrap(),
        Expression::Definition(s, e) => analyze_definition(s, e).unwrap(),
        Expression::If(e1, e2, e3) => analyze_if(e1, e2, e3).unwrap(),
        Expression::Lambda(e1, e2) => analyze_lambda(e1, e2).unwrap(),
        Expression::Begin(e) => analyze_sequence(e).unwrap(),
        Expression::Cond(e) => analyze(cond_to_if(e).unwrap()),
        Expression::Application(e1, e2) => analyze_application(e1, e2).unwrap(),
    }
}

fn analyze_self_evaluated(
    v: SelfEval,
) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    Ok(Box::new(|env| Ok(Value::Atom(v))))
}

fn analyze_quoted(e: Sexpr) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    Ok(Box::new(|env| match e {
        Sexpr::Atom(a) => Ok(Value::Atom(a)),
        Sexpr::Cons(c) => Ok(Value::Cons(c)),
    }))
}

fn analyze_variable(s: Symbol) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    todo!()
}

fn analyze_assignment(
    s: Symbol,
    e: Sexpr,
) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    todo!()
}

fn analyze_definition(
    s: Symbol,
    e: Sexpr,
) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    todo!()
}

fn analyze_if(
    e1: Sexpr,
    e2: Sexpr,
    e3: Sexpr,
) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    todo!()
}

fn analyze_lambda(
    e1: Sexpr,
    e2: Sexpr,
) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    Ok(Box::new(|env| {
        Ok(Value::Procedure(Procedure::Compound(CompoundProcedure {
            parameters: match e1 {
                Sexpr::Cons(v) => v,
                _ => return Err(()),
            },
            body: match e2 {
                Sexpr::Cons(v) => v,
                _ => return Err(()),
            },
            env: env.clone(),
        })))
    }))
}

fn analyze_sequence(e: Sexpr) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    todo!()
}

fn cond_to_if(e: Sexpr) -> Result<Expression, ()> {
    todo!()
}

fn analyze_application(
    e1: Sexpr,
    e2: Sexpr,
) -> Result<Box<dyn FnOnce(&mut Env) -> Result<Value, ()>>, ()> {
    todo!()
}
