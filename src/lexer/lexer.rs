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
        while Self::isLetter(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    pub fn skip_whitepace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::isDigit(self.ch) {
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

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token = Token::New();

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
                if Self::isLetter(self.ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = Token::lookup_identifier(tok.literal.to_string());
                    return tok;
                } else if Self::isDigit(self.ch) {
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

    pub fn isLetter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    pub fn isDigit(ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }
}
