use crate::lexer::lexer::*;
use crate::token::token::*;
use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        let mut line: String = String::new();
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        let mut l = Lexer::new(line);

        let mut tok = l.next_token();

        while (tok.token_type
            != TokenType {
                token_type: EOF.to_string(),
            })
        {
            print!("{}\n", tok);

            tok = l.next_token();
        }
        break;
    }
}
