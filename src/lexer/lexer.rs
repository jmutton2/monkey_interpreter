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

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '0';
        } else {
            return self.input.as_bytes()[self.read_position] as char;
        }
    }

    pub fn skip_whitepace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token = Token::new();

        self.skip_whitepace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let new_string = format!("{}{}", ch, self.ch);
                    print!("{:?}", new_string);

                    tok = Token::new_token(EQUALS.to_string(), new_string);
                } else {
                    tok = Token::new_token(ASSIGN.to_string(), self.ch.to_string())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let new_string = format!("{}{}", ch, self.ch);
                    print!("{:?}", new_string);

                    tok = Token::new_token(NOT_EQUALS.to_string(), new_string);
                } else {
                    tok = Token::new_token(BANG.to_string(), self.ch.to_string());
                }
            }

            ';' => tok = Token::new_token(SEMICOLON.to_string(), self.ch.to_string()),
            '(' => tok = Token::new_token(LPAREN.to_string(), self.ch.to_string()),
            ')' => tok = Token::new_token(RPAREN.to_string(), self.ch.to_string()),
            '{' => tok = Token::new_token(LBRACE.to_string(), self.ch.to_string()),
            '}' => tok = Token::new_token(RBRACE.to_string(), self.ch.to_string()),
            ',' => tok = Token::new_token(COMMA.to_string(), self.ch.to_string()),
            '+' => tok = Token::new_token(PLUS.to_string(), self.ch.to_string()),
            '-' => tok = Token::new_token(MINUS.to_string(), self.ch.to_string()),
            '/' => tok = Token::new_token(SLASH.to_string(), self.ch.to_string()),
            '*' => tok = Token::new_token(ASTERISK.to_string(), self.ch.to_string()),
            '<' => tok = Token::new_token(LT.to_string(), self.ch.to_string()),
            '>' => tok = Token::new_token(GT.to_string(), self.ch.to_string()),
            '0' => tok = Token::new_token(EOF.to_string(), "".to_string()),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    tok.literal = self.read_identifier();
                    tok.token_type = Token::lookup_identifier(tok.literal.to_string());
                    return tok;
                } else if self.ch.is_ascii_digit() {
                    tok = Token::new_token(INT.to_string(), self.read_number());
                    return tok;
                } else {
                    tok = Token::new_token(ILLEGAL.to_string(), self.ch.to_string())
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

    let tests = [
        TokenType::Let,
        TokenType::Ident(String::from("five")),
        TokenType::Assign,
        TokenType::Int(String::from("5")),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident(String::from("ten")),
        TokenType::Assign,
        TokenType::Int(String::from("10")),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident(String::from("add")),
        TokenType::Assign,
        TokenType::Function,
        TokenType::Lparen,
        TokenType::Ident(String::from("x")),
        TokenType::Comma,
        TokenType::Ident(String::from("y")),
        TokenType::Rparen,
        TokenType::Lbrace,
        TokenType::Ident(String::from("x")),
        TokenType::Plus,
        TokenType::Ident(String::from("y")),
        TokenType::Semicolon,
        TokenType::Rbrace,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident(String::from("result")),
        TokenType::Assign,
        TokenType::Ident(String::from("add")),
        TokenType::Lparen,
        TokenType::Ident(String::from("five")),
        TokenType::Comma,
        TokenType::Ident(String::from("ten")),
        TokenType::Rparen,
        TokenType::Semicolon,
        TokenType::Bang,
        TokenType::Minus,
        TokenType::Slash,
        TokenType::Asterisk,
        TokenType::Int(String::from("5")),
        TokenType::Semicolon,
        TokenType::Int(String::from("5")),
        TokenType::Lt,
        TokenType::Int(String::from("10")),
        TokenType::Gt,
        TokenType::Int(String::from("5")),
        TokenType::Semicolon,
        TokenType::If,
        TokenType::Lparen,
        TokenType::Int(String::from("5")),
        TokenType::Lt,
        TokenType::Int(String::from("10")),
        TokenType::Rparen,
        TokenType::Lbrace,
        TokenType::Return,
        TokenType::True,
        TokenType::Semicolon,
        TokenType::Rbrace,
        TokenType::Else,
        TokenType::Lbrace,
        TokenType::Return,
        TokenType::False,
        TokenType::Semicolon,
        TokenType::Rbrace,
        TokenType::Int(String::from("10")),
        TokenType::Equals,
        TokenType::Int(String::from("10")),
        TokenType::Semicolon,
        TokenType::Int(String::from("10")),
        TokenType::Noteqauls,
        TokenType::Int(String::from("9")),
        TokenType::Semicolon,
        TokenType::Eof,
    ];

    let mut lex = Lexer::new(input);

    for token in tests {
        let next_token = lex.next_token();
        assert_eq!(Token::process_token(token), next_token);
    }
}
