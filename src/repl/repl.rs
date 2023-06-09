use crate::lexer::lexer::*;
use crate::token::token::*;
use std::io::Write;

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        let mut line: String = String::new();
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();

        let mut l = Lexer::new(line);

        let mut tok = l.next_token();

        while tok != Token::Eof {
            print!("{}\n", tok);

            tok = l.next_token();
        }
        break;
    }
}
