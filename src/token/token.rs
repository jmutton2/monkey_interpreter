pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const SLASH: &str = "/";
pub const ASTERISK: &str = "*";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";
pub const BANG: &str = "!";
pub const EQUALS: &str = "==";
pub const NOT_EQUALS: &str = "!=";
pub const LT: &str = "<";
pub const GT: &str = ">";

pub const KEYWORDS: [(&str, &str); 7] = [
    ("fn", FUNCTION),
    ("let", LET),
    ("true", TRUE),
    ("false", FALSE),
    ("if", IF),
    ("else", ELSE),
    ("return", RETURN),
];

#[derive(Debug)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Minus,
    Slash,
    Asterisk,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Bang,
    Equals,
    Noteqauls,
    Lt,
    Gt,
}

impl Token {
    pub fn lookup_identifier(ident: String) -> Token {
        let value = KEYWORDS.iter().find(|&(k, _)| *k == ident).map(|&(_, v)| v);
        if value.is_none() {
            return Token::Ident(ident);
        }

        match value.unwrap() {
            FUNCTION => Token::Function,
            LET => Token::Let,
            TRUE => Token::True,
            FALSE => Token::False,
            IF => Token::If,
            ELSE => Token::Else,
            RETURN => Token::Return,
            _ => Token::Illegal,
        }
    }

    pub fn as_literal(&self) -> String {
        match self {
            Token::Illegal => String::from(""),
            Token::Eof => String::from(""),
            Token::Ident(name) => name.to_owned(),
            Token::Int(value) => value.to_owned(),
            Token::Assign => String::from("="),
            Token::Plus => String::from("+"),
            Token::Minus => String::from("-"),
            Token::Slash => String::from("/"),
            Token::Asterisk => String::from("*"),
            Token::Comma => String::from(","),
            Token::Semicolon => String::from(";"),
            Token::Lparen => String::from("("),
            Token::Rparen => String::from(")"),
            Token::Lbrace => String::from("{"),
            Token::Rbrace => String::from("}"),
            Token::Function => String::from("function"),
            Token::Let => String::from("let"),
            Token::True => String::from("true"),
            Token::False => String::from("false"),
            Token::If => String::from("if"),
            Token::Else => String::from("else"),
            Token::Return => String::from("return"),
            Token::Bang => String::from("!"),
            Token::Equals => String::from("=="),
            Token::Noteqauls => String::from("!="),
            Token::Lt => String::from("<"),
            Token::Gt => String::from(">"),
        }
    }

    pub fn is_token_ident(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Ident(ident1), Token::Ident(ident2)) => true,
            _ => false
        }
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Illegal, Token::Illegal)
            | (Token::Eof, Token::Eof)
            | (Token::Assign, Token::Assign)
            | (Token::Plus, Token::Plus)
            | (Token::Minus, Token::Minus)
            | (Token::Slash, Token::Slash)
            | (Token::Asterisk, Token::Asterisk)
            | (Token::Comma, Token::Comma)
            | (Token::Semicolon, Token::Semicolon)
            | (Token::Lparen, Token::Lparen)
            | (Token::Rparen, Token::Rparen)
            | (Token::Lbrace, Token::Lbrace)
            | (Token::Rbrace, Token::Rbrace)
            | (Token::Function, Token::Function)
            | (Token::Let, Token::Let)
            | (Token::True, Token::True)
            | (Token::False, Token::False)
            | (Token::If, Token::If)
            | (Token::Else, Token::Else)
            | (Token::Return, Token::Return)
            | (Token::Bang, Token::Bang)
            | (Token::Equals, Token::Equals)
            | (Token::Noteqauls, Token::Noteqauls)
            | (Token::Lt, Token::Lt)
            | (Token::Gt, Token::Gt) => true,
            (Token::Ident(ident1), Token::Ident(ident2))
            | (Token::Int(ident1), Token::Int(ident2)) => ident1 == ident2,
            _ => false,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => write!(f, "{{ Type: {}\tLiteral: {} }}", ILLEGAL, ""),
            Token::Eof => write!(f, "{{ Type: {}\tLiteral: {} }}", EOF, ""),
            Token::Ident(name) => write!(f, "{{ Type: {}\tLiteral: {} }}", IDENT, name),
            Token::Int(value) => write!(f, "{{ Type: {}\tLiteral: {} }}", INT, value),
            Token::Assign => write!(f, "{{ Type: {}\tLiteral: {} }}", ASSIGN, "="),
            Token::Plus => write!(f, "{{ Type: {}\tLiteral: {} }}", PLUS, "+"),
            Token::Minus => write!(f, "{{ Type: {}\tLiteral: {} }}", MINUS, "-"),
            Token::Slash => write!(f, "{{ Type: {}\tLiteral: {} }}", SLASH, "/"),
            Token::Asterisk => write!(f, "{{ Type: {}\tLiteral: {} }}", ASTERISK, "*"),
            Token::Comma => write!(f, "{{ Type: {}\tLiteral: {} }}", COMMA, ","),
            Token::Semicolon => write!(f, "{{ Type: {}\tLiteral: {} }}", SEMICOLON, ";"),
            Token::Lparen => write!(f, "{{ Type: {}\tLiteral: {} }}", LPAREN, "("),
            Token::Rparen => write!(f, "{{ Type: {}\tLiteral: {} }}", RPAREN, ")"),
            Token::Lbrace => write!(f, "{{ Type: {}\tLiteral: {} }}", LBRACE, "{"),
            Token::Rbrace => write!(f, "{{ Type: {}\tLiteral: {} }}", RBRACE, "}"),
            Token::Function => write!(f, "{{ Type: {}\tLiteral: {} }}", FUNCTION, "function"),
            Token::Let => write!(f, "{{ Type: {}\tLiteral: {} }}", LET, "let"),
            Token::True => write!(f, "{{ Type: {}\tLiteral: {} }}", TRUE, "true"),
            Token::False => write!(f, "{{ Type: {}\tLiteral: {} }}", FALSE, "false"),
            Token::If => write!(f, "{{ Type: {}\tLiteral: {} }}", IF, "if"),
            Token::Else => write!(f, "{{ Type: {}\tLiteral: {} }}", ELSE, "else"),
            Token::Return => write!(f, "{{ Type: {}\tLiteral: {} }}", RETURN, "return"),
            Token::Bang => write!(f, "{{ Type: {}\tLiteral: {} }}", BANG, "!"),
            Token::Equals => write!(f, "{{ Type: {}\tLiteral: {} }}", EQUALS, "=="),
            Token::Noteqauls => write!(f, "{{ Type: {}\tLiteral: {} }}", NOT_EQUALS, "!="),
            Token::Lt => write!(f, "{{ Type: {}\tLiteral: {} }}", LT, "<"),
            Token::Gt => write!(f, "{{ Type: {}\tLiteral: {} }}", GT, ">"),
        }
    }
}
