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
    pub parameters: Cons,
    pub body: Cons,
    pub env: Env,
}

#[derive(Clone, Debug)]
pub enum Value {
    Atom(Atom),
    Cons(Cons),
    Procedure(Procedure),
}

#[derive(Clone, Debug)]
pub enum LambdaParam {
    SelfEval(SelfEval),
    Sexpr(Sexpr),
    Symbol(Symbol),
    SymbolAndSexpr(Symbol, Sexpr),
    ThreeSexpr(Sexpr, Sexpr, Sexpr),
    TwoSexpr(Sexpr, Sexpr),
}

pub fn analyze(
    exp: Expression,
) -> Result<(fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam), ()> {
    match exp {
        Expression::SelfEval(v) => Ok(analyze_self_evaluated(v)),
        Expression::Quoted(v) => Ok(analyze_quoted(v)),
        Expression::Variable(v) => Ok(analyze_variable(v)),
        Expression::Assignment(s, e) => Ok(analyze_assignment(s, e)),
        Expression::Definition(s, e) => Ok(analyze_definition(s, e)),
        Expression::If(e1, e2, e3) => Ok(analyze_if(e1, e2, e3)),
        Expression::Lambda(e1, e2) => Ok(analyze_lambda(e1, e2)),
        Expression::Begin(e) => Ok(analyze_sequence(e)),
        Expression::Cond(e) => match cond_to_if(e) {
            Ok(e) => analyze(e),
            Err(_) => Err(()),
        },
        Expression::Application(e1, e2) => Ok(analyze_application(e1, e2)),
    }
}

fn analyze_self_evaluated(
    v: SelfEval,
) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        match p {
            LambdaParam::SelfEval(v) => Ok(Value::Atom(v)),
            _ => Err(()),
        }
    }
    (helper, LambdaParam::SelfEval(v))
}

fn analyze_quoted(e: Sexpr) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        match p {
            LambdaParam::Sexpr(s) => match s {
                Sexpr::Atom(a) => Ok(Value::Atom(a)),
                Sexpr::Cons(c) => Ok(Value::Cons(c)),
            },
            _ => Err(()),
        }
    }
    (helper, LambdaParam::Sexpr(e))
}

fn analyze_variable(s: Symbol) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        todo!()
    }
    (helper, LambdaParam::Symbol(s))
}

fn analyze_assignment(
    s: Symbol,
    e: Sexpr,
) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        todo!()
    }
    (helper, LambdaParam::SymbolAndSexpr(s, e))
}

fn analyze_definition(
    s: Symbol,
    e: Sexpr,
) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        todo!()
    }
    (helper, LambdaParam::SymbolAndSexpr(s, e))
}

fn analyze_if(
    e1: Sexpr,
    e2: Sexpr,
    e3: Sexpr,
) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        todo!()
    }
    (helper, LambdaParam::ThreeSexpr(e1, e2, e3))
}

fn analyze_lambda(
    e1: Sexpr,
    e2: Sexpr,
) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    // Ok(Box::new(|env| {
    //     Ok(Value::Procedure(Procedure::Compound(CompoundProcedure {
    //         parameters: match e1 {
    //             Sexpr::Cons(v) => v,
    //             _ => return Err(()),
    //         },
    //         body: match e2 {
    //             Sexpr::Cons(v) => v,
    //             _ => return Err(()),
    //         },
    //         env: env.clone(),
    //     })))
    // }))
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        todo!()
    }
    (helper, LambdaParam::TwoSexpr(e1, e2))
}

fn analyze_sequence(e: Sexpr) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        todo!()
    }
    (helper, LambdaParam::Sexpr(e))
}

fn cond_to_if(e: Sexpr) -> Result<Expression, ()> {
    todo!()
}

fn analyze_application(
    e1: Sexpr,
    e2: Sexpr,
) -> (fn(&mut Env, LambdaParam) -> Result<Value, ()>, LambdaParam) {
    fn helper(env: &mut Env, p: LambdaParam) -> Result<Value, ()> {
        todo!()
    }
    (helper, LambdaParam::TwoSexpr(e1, e2))
}
