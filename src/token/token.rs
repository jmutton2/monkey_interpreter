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

// pub struct TokenType {
//     pub token_type: String,
// }

#[derive(Debug)]
pub struct Token {
    // pub token_type: TokenType,
    pub token_type: String,
    pub literal: String,
}

struct TokenStruct {
    token_type: String,
    literal: String,
}

pub enum TokenType {
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
    pub fn new() -> Token {
        // Token {
        //     token_type: TokenType {
        //         token_type: EOF.to_string(),
        //     },
        //     literal: EOF.to_string(),
        // }
        Self::process_token(TokenType::Eof)
    }

    pub fn process_token(token: TokenType) -> Token {
        match token {
            TokenType::Illegal => Token {
                token_type: String::from(ILLEGAL),
                literal: String::from(""),
            },
            TokenType::Eof => Token {
                token_type: String::from(EOF),
                literal: String::from(""),
            },
            TokenType::Ident(value) => Token {
                token_type: String::from(IDENT),
                literal: String::from(value),
            },
            TokenType::Int(value) => Token {
                token_type: String::from(INT),
                literal: String::from(value),
            },
            TokenType::Assign => Token {
                token_type: String::from(ASSIGN),
                literal: String::from("="),
            },
            TokenType::Plus => Token {
                token_type: String::from(PLUS),
                literal: String::from("+"),
            },
            TokenType::Minus => Token {
                token_type: String::from(MINUS),
                literal: String::from("-"),
            },
            TokenType::Slash => Token {
                token_type: String::from(SLASH),
                literal: String::from("/"),
            },
            TokenType::Asterisk => Token {
                token_type: String::from(ASTERISK),
                literal: String::from("*"),
            },
            TokenType::Comma => Token {
                token_type: String::from(COMMA),
                literal: String::from(","),
            },
            TokenType::Semicolon => Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            TokenType::Lparen => Token {
                token_type: String::from(LPAREN),
                literal: String::from("("),
            },
            TokenType::Rparen => Token {
                token_type: String::from(RPAREN),
                literal: String::from(")"),
            },
            TokenType::Lbrace => Token {
                token_type: String::from(LBRACE),
                literal: String::from("{"),
            },
            TokenType::Rbrace => Token {
                token_type: String::from(RBRACE),
                literal: String::from("}"),
            },
            TokenType::Function => Token {
                token_type: String::from(FUNCTION),
                literal: String::from("fn"),
            },
            TokenType::Let => Token {
                token_type: String::from(LET),
                literal: String::from("let"),
            },
            TokenType::True => Token {
                token_type: String::from(TRUE),
                literal: String::from("true"),
            },
            TokenType::False => Token {
                token_type: String::from(FALSE),
                literal: String::from("false"),
            },
            TokenType::If => Token {
                token_type: String::from(IF),
                literal: String::from("if"),
            },
            TokenType::Else => Token {
                token_type: String::from(ELSE),
                literal: String::from("else"),
            },
            TokenType::Return => Token {
                token_type: String::from(RETURN),
                literal: String::from("return"),
            },
            TokenType::Bang => Token {
                token_type: String::from(BANG),
                literal: String::from("!"),
            },
            TokenType::Equals => Token {
                token_type: String::from(EQUALS),
                literal: String::from("=="),
            },
            TokenType::Noteqauls => Token {
                token_type: String::from(NOT_EQUALS),
                literal: String::from("!="),
            },
            TokenType::Lt => Token {
                token_type: String::from(LT),
                literal: String::from("<"),
            },
            TokenType::Gt => Token {
                token_type: String::from(GT),
                literal: String::from(">"),
            },
        }
    }

    pub fn new_token(token_type: String, literal: String) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }

    pub fn lookup_identifier(ident: String) -> String {
        let value = KEYWORDS.iter().find(|&(k, _)| *k == ident).map(|&(_, v)| v);
        match value {
            Some(value) => value.to_string(),
            _ => IDENT.to_string(),
        }
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        self.literal == other.literal && self.token_type == other.token_type
    }
}

// impl std::fmt::Display for TokenType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.token_type)
//     }
// }

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Type: {} Literal: {}}}", self.token_type, self.literal)
    }
}
