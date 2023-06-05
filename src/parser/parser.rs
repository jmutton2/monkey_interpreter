use crate::ast::ast::*;
use crate::lexer::lexer::*;
use crate::token::token::*;

pub struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }

    fn parse_program(&mut self) -> Program {
        return Program {
            statements: Vec::new(),
        };
    }
}

pub fn new(l: Lexer) -> Parser {
    let mut p = Parser {
        l: l,
        current_token: Token::new(),
        peek_token: Token::new(),
    };

    p.next_token();
    p.next_token();

    return p;
}
