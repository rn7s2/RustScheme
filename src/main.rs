#![feature(fn_traits)]

mod analyzer;
mod evaluator;
mod lexer;
mod parser;

use std::collections::HashMap;

use analyzer::{analyze, Value};
use evaluator::{eval, extend_env, Env, THE_EMPTY_ENV};
use lexer::{format_sexpr, lex, read_sexpr, Symbol};
use parser::parse;

const INPUT_PROMPT: &str = ";;; Scheme input:";
const OUTPUT_PROMPT: &str = ";;; Scheme value:";

fn main() {
    let global_env = setup_env();
    driver_loop(global_env);
}

fn setup_env() -> Env {
    let mut init_env = extend_env(primitive_procs(), &THE_EMPTY_ENV);
    // define_variable();
    // define_variable();
    // init_env
    todo!()
}

fn driver_loop(env: Env) {
    loop {
        prompt_for_input(INPUT_PROMPT);
        let mut inner_env = env.clone();
        eval(read_sexpr().unwrap());

        match read_sexpr() {
            Ok(s) => match parse(s) {
                Ok(e) => match eval(e, env) {
                    Ok(v) => {
                        announce_output(OUTPUT_PROMPT);
                        user_print(v);
                    }
                    Err(_) => driver_loop(env),
                },
                Err(_) => driver_loop(env),
            },
            Err(_) => driver_loop(env),
        }
    }
}

fn prompt_for_input(str: &str) {}

fn announce_output(str: &str) {}

fn user_print(obj: Value) {}
