use crate::ast::ast::*;
use crate::lexer::lexer::*;
use crate::token::token::*;

pub struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p: Parser = Parser {
            l: l,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new(),
        };

        p.next_token();
        p.next_token();

        return p;
    }

    fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };
        while !self.current_token_is(&Token::Eof) {
            let stmt = self.parse_statement();
            if stmt != StatementType::NilStatement {
                let mut vec = vec![stmt];
                program.statements.append(&mut vec)
            }
            self.next_token();
        }
        return program;
    }

    fn parse_statement(&mut self) -> StatementType {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => StatementType::NilStatement,
        }
    }

    fn parse_let_statement(&mut self) -> StatementType {
        let stmt: StatementType;

        if !self.expect_peek(Token::Ident(String::from(""))) {
            return StatementType::NilStatement;
        }

        stmt = StatementType::LetStatement(LetStatement {
            token: Token::Let,
            name: Identifier {
                token: Token::Let,
                value: self.current_token.as_literal(),
            },
            value: Expression { node: Node {} },
        });

        if !self.expect_peek(Token::Assign) {
            return StatementType::NilStatement;
        }

        while !self.current_token_is(&Token::Semicolon) {
            self.next_token();
        }

        return stmt;
    }

    fn parse_return_statement(&mut self) -> StatementType {
        let stmt: StatementType = StatementType::ReturnStatement(ReturnStatement {
            token: Token::Return,
            value: Expression { node: Node {} },
        });

        self.next_token();

        return stmt;
    }

    fn current_token_is(&mut self, t: &Token) -> bool {
        return self.current_token == *t || self.current_token.is_token_ident(&t);
    }

    fn peek_token_is(&mut self, t: &Token) -> bool {
        return self.peek_token == *t || self.peek_token.is_token_ident(&t);
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!(
            "Expected next token to be {}, got {} instead.",
            t,
            self.peek_token.as_literal()
        );
        self.errors.push(msg);
    }

    pub fn check_parse_errors(p: Parser) {
        let errors = p.errors;

        if errors.len() == 0 {
            return;
        }

        println!("Parser had {} errors.", errors.len());

        for msg in errors {
            println!("{}", msg);
        }

        assert_eq!(true, false);
    }
}

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    "
    .to_string();

    let lex: Lexer = Lexer::new(input);
    let mut p: Parser = Parser::new(lex);

    let program: Program = p.parse_program();
    Parser::check_parse_errors(p);

    assert_eq!(program.statements.len(), 3);

    let tests = vec!["x", "y", "foobar"];

    for i in 0..tests.len() {
        let stmt = &program.statements[i];

        if !test_let_statement(stmt, tests[i].to_string()) {
            return;
        }
    }
}

fn test_let_statement(s: &StatementType, name: String) -> bool {
    assert_eq!(s.token_literal(), "let".to_string());

    let stmt = StatementType::LetStatement(LetStatement {
        token: Token::Let,
        name: Identifier {
            token: Token::Let,
            value: name,
        },
        value: Expression { node: { Node {} } },
    });

    assert_eq!(s, &stmt);

    return true;
}

#[test]
fn test_retun_statements() {
    let input = "
        return 5;
        return 10;
        return 993322;
    "
    .to_string();

    let lex = Lexer::new(input);
    let mut p: Parser = Parser::new(lex);

    let program = p.parse_program();

    assert_eq!(program.statements.len(), 3);

    for stmt in program.statements {
        assert_eq!(
            stmt,
            StatementType::ReturnStatement(ReturnStatement {
                token: Token::Return,
                value: Expression { node: Node {} }
            })
        );
    }
}
