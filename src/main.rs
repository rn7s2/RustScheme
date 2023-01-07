mod lexer;
mod parser;

use lexer::lex;
use parser::parse;

fn main() {
    println!("{:?}", parse(lex("".to_owned()).unwrap()));
}
