pub const ILLEGAL: String = "ILLEGAL".to_string();
pub const EOF: String = "EOF".to_string();

pub const IDENT: String = "IDENT".to_string();
pub const INT: String = "INT".to_string();

pub const ASSIGN: String = "=".to_string();
pub const PLUS: String = "+".to_string();

pub const COMMA: String = ",".to_string();
pub const SEMICOLON: String = ";".to_string();

pub const LPAREN: String = "(".to_string();
pub const RPAREN: String = ")".to_string();
pub const LBRACE: String = "{".to_string();
pub const RBRACE: String = "}".to_string();

pub const FUNCTION: String = "FUNCTION".to_string();
pub const LET: String = "LET".to_string();


pub struct TokenType {
    token_type: String
}

pub struct Token {
    token_type: TokenType,
    literal: String,
}
