use crate::token::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '0';
        } else {
            return self.input.as_bytes()[self.read_position] as char;
        }
    }

    fn skip_whitepace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitepace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::Equals;
                } else {
                    tok = Token::Assign;
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::Noteqauls;
                } else {
                    tok = Token::Bang;
                }
            }

            ';' => tok = Token::Semicolon,
            '(' => tok = Token::Lparen,
            ')' => tok = Token::Rparen,
            '{' => tok = Token::Lbrace,
            '}' => tok = Token::Rbrace,
            ',' => tok = Token::Comma,
            '+' => tok = Token::Plus,
            '-' => tok = Token::Minus,
            '/' => tok = Token::Slash,
            '*' => tok = Token::Asterisk,
            '<' => tok = Token::Lt,
            '>' => tok = Token::Gt,
            '0' => tok = Token::Eof,
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let literal = self.read_identifier();
                    tok = Token::lookup_identifier(literal.to_string());
                    return tok;
                } else if self.ch.is_ascii_digit() {
                    tok = Token::Int(self.read_number());
                    return tok;
                } else {
                    tok = Token::Illegal;
                }
            }
        }

        self.read_char();
        return tok;
    }
}

#[test]
pub fn test_next_token() {
    let input = "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
            } else {
            return false;
        }

        10 == 10;
        10 != 9; 
    "
    .to_string();

    let tests = vec![
        Token::Let,
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::Lparen,
        Token::Ident(String::from("x")),
        Token::Comma,
        Token::Ident(String::from("y")),
        Token::Rparen,
        Token::Lbrace,
        Token::Ident(String::from("x")),
        Token::Plus,
        Token::Ident(String::from("y")),
        Token::Semicolon,
        Token::Rbrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("result")),
        Token::Assign,
        Token::Ident(String::from("add")),
        Token::Lparen,
        Token::Ident(String::from("five")),
        Token::Comma,
        Token::Ident(String::from("ten")),
        Token::Rparen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Int(String::from("5")),
        Token::Lt,
        Token::Int(String::from("10")),
        Token::Gt,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::If,
        Token::Lparen,
        Token::Int(String::from("5")),
        Token::Lt,
        Token::Int(String::from("10")),
        Token::Rparen,
        Token::Lbrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::Rbrace,
        Token::Else,
        Token::Lbrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::Rbrace,
        Token::Int(String::from("10")),
        Token::Equals,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Int(String::from("10")),
        Token::Noteqauls,
        Token::Int(String::from("9")),
        Token::Semicolon,
        Token::Eof,
    ];

    let mut lex = Lexer::new(input);

    for token in tests {
        let next_token = lex.next_token();
        assert_eq!(token, next_token);
    }
}
