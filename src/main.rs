#![feature(fn_traits)]

mod analyzer;
mod evaluator;
mod lexer;
mod parser;

use analyzer::Value;
use evaluator::{define_variable, eval, extend_env, primitive_procs, Env, THE_EMPTY_ENV};
use lexer::{read_sexpr, Atom, Bool, Symbol};
use parser::parse;

const INPUT_PROMPT: &str = ";;; Scheme input:";
const OUTPUT_PROMPT: &str = ";;; Scheme value:";

fn main() {
    driver_loop(setup_env());
}

fn setup_env() -> Env {
    let mut init_env = extend_env(primitive_procs(), &THE_EMPTY_ENV);
    define_variable(
        Symbol("true".to_owned()),
        Value::Atom(Atom::Bool(Bool::True)),
        &mut init_env,
    );
    define_variable(
        Symbol("false".to_owned()),
        Value::Atom(Atom::Bool(Bool::False)),
        &mut init_env,
    );
    init_env
}

fn driver_loop(mut env: Env) {
    loop {
        prompt_for_input(INPUT_PROMPT);
        let val = eval(parse(read_sexpr().unwrap()).unwrap(), &mut env).unwrap();
        announce_output(OUTPUT_PROMPT);
        user_print(val);
    }
}

fn prompt_for_input(str: &str) {
    println!("\n\n{}", str);
}

fn announce_output(str: &str) {
    println!("\n{}", str);
}

fn user_print(val: Value) {
    print!("{:?}", val);
}
