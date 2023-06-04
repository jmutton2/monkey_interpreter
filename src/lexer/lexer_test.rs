use crate::token::token::*;

struct TestToken {
    expected_type: TokenType,
    expected_literal: String
}

fn test_next_token() {
    let input = "=+(){},;";

    let tests = [ 
        TestToken { expected_type: TokenType { token_type: ASSIGN }, expected_literal: "=".to_string() },
        TestToken { expected_type: TokenType { token_type: PLUS }, expected_literal: "+".to_string() },
        TestToken { expected_type: TokenType { token_type: LPAREN }, expected_literal: "(".to_string()},
        TestToken { expected_type: TokenType { token_type: RPAREN }, expected_literal: ")".to_string()},
        TestToken { expected_type: TokenType { token_type: LBRACE }, expected_literal: "{".to_string()},
        TestToken { expected_type: TokenType { token_type: RBRACE }, expected_literal: "}".to_string()},
        TestToken { expected_type: TokenType { token_type: COMMA }, expected_literal: ",".to_string()},
        TestToken { expected_type: TokenType { token_type: SEMICOLON }, expected_literal: ";".to_string()},
        TestToken { expected_type: TokenType { token_type: EOF }, expected_literal: "".to_string()},
    ];

    l = new(input);

    for test in tests.iter() {
        let tok = l.next_token();
        
        assert_eq!(tok.Type, l.expected_type);
        assert_eq!(tok.Literal, l.expected_literal);
    }
}
