mod lexer;
mod repl;
mod token;

use crate::repl::repl::start;

fn main() {
    println!("Welcome to the Monkey Interpreter. Made blazingly fast with Rust!");
    start();
}
