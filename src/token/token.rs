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
pub struct TokenType {
    pub token_type: String,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new() -> Token {
        Token {
            token_type: TokenType {
                token_type: EOF.to_string(),
            },
            literal: EOF.to_string(),
        }
    }

    pub fn new_token(token_type: String, literal: String) -> Token {
        Token {
            token_type: TokenType {
                token_type: token_type,
            },
            literal: literal,
        }
    }

    pub fn lookup_identifier(ident: String) -> TokenType {
        let value = KEYWORDS.iter().find(|&(k, _)| *k == ident).map(|&(_, v)| v);
        match value {
            Some(value) => TokenType {
                token_type: value.to_string(),
            },
            _ => TokenType {
                token_type: IDENT.to_string(),
            },
        }
    }
}

impl PartialEq<TokenType> for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == other.token_type
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Type: {} Literal: {}}}", self.token_type, self.literal)
    }
}
