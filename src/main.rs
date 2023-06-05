mod lexer;
mod repl;
mod token;

use crate::lexer::*;
use crate::repl::repl::start;
use crate::token::*;

fn main() {
    println!("Hello, world!");
    start();
}
