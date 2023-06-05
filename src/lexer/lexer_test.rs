use crate::lexer::lexer::*;
use crate::token::token::*;

struct TestToken {
    expected_type: TokenType,
    expected_literal: String,
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
        TestToken {
            expected_type: TokenType {
                token_type: LET.to_string(),
            },
            expected_literal: "let".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "five".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: ASSIGN.to_string(),
            },
            expected_literal: "=".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: INT.to_string(),
            },
            expected_literal: "5".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: SEMICOLON.to_string(),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: LET.to_string(),
            },
            expected_literal: "let".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "ten".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: ASSIGN.to_string(),
            },
            expected_literal: "=".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: INT.to_string(),
            },
            expected_literal: "10".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: SEMICOLON.to_string(),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: LET.to_string(),
            },
            expected_literal: "let".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "add".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: ASSIGN.to_string(),
            },
            expected_literal: "=".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: FUNCTION.to_string(),
            },
            expected_literal: "fn".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: LPAREN.to_string(),
            },
            expected_literal: "(".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "x".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: COMMA.to_string(),
            },
            expected_literal: ",".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "y".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: RPAREN.to_string(),
            },
            expected_literal: ")".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: LBRACE.to_string(),
            },
            expected_literal: "{".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "x".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: PLUS.to_string(),
            },
            expected_literal: "+".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "y".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: SEMICOLON.to_string(),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: RBRACE.to_string(),
            },
            expected_literal: "}".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: SEMICOLON.to_string(),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: LET.to_string(),
            },
            expected_literal: "let".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "result".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: ASSIGN.to_string(),
            },
            expected_literal: "=".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "add".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: LPAREN.to_string(),
            },
            expected_literal: "(".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "five".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: COMMA.to_string(),
            },
            expected_literal: ",".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: IDENT.to_string(),
            },
            expected_literal: "ten".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: RPAREN.to_string(),
            },
            expected_literal: ")".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: SEMICOLON.to_string(),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (BANG.to_string()),
            },
            expected_literal: "!".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (MINUS.to_string()),
            },
            expected_literal: "-".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (SLASH.to_string()),
            },
            expected_literal: "/".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (ASTERISK.to_string()),
            },
            expected_literal: "*".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (INT.to_string()),
            },
            expected_literal: "5".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (SEMICOLON.to_string()),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (INT.to_string()),
            },
            expected_literal: "5".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (LT.to_string()),
            },
            expected_literal: "<".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (INT.to_string()),
            },
            expected_literal: "10".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (GT.to_string()),
            },
            expected_literal: ">".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (INT.to_string()),
            },
            expected_literal: "5".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (SEMICOLON.to_string()),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (IF.to_string()),
            },
            expected_literal: "if".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (LPAREN.to_string()),
            },
            expected_literal: "(".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (INT.to_string()),
            },
            expected_literal: "5".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (LT.to_string()),
            },
            expected_literal: "<".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (INT.to_string()),
            },
            expected_literal: "10".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (RPAREN.to_string()),
            },
            expected_literal: ")".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (LBRACE.to_string()),
            },
            expected_literal: "{".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (RETURN.to_string()),
            },
            expected_literal: "return".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (TRUE.to_string()),
            },
            expected_literal: "true".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (SEMICOLON.to_string()),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (RBRACE.to_string()),
            },
            expected_literal: "}".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (ELSE.to_string()),
            },
            expected_literal: "else".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (LBRACE.to_string()),
            },
            expected_literal: "{".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (RETURN.to_string()),
            },
            expected_literal: "return".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (FALSE.to_string()),
            },
            expected_literal: "false".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (SEMICOLON.to_string()),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: (RBRACE.to_string()),
            },
            expected_literal: "}".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: INT.to_string(),
            },
            expected_literal: "10".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: EQUALS.to_string(),
            },
            expected_literal: "==".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: INT.to_string(),
            },
            expected_literal: "10".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: SEMICOLON.to_string(),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: INT.to_string(),
            },
            expected_literal: "10".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: NOT_EQUALS.to_string(),
            },
            expected_literal: "!=".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: INT.to_string(),
            },
            expected_literal: "9".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: SEMICOLON.to_string(),
            },
            expected_literal: ";".to_string(),
        },
        TestToken {
            expected_type: TokenType {
                token_type: EOF.to_string(),
            },
            expected_literal: "".to_string(),
        },
    ];

    let mut l = Lexer::new(input);

    for test in tests.iter() {
        let tok = l.next_token();
        println!(
            "{:?},{:?},{:?},{:?} ",
            tok.token_type, test.expected_type, tok.literal, test.expected_literal
        );
        assert_eq!(tok.token_type, test.expected_type);
        assert_eq!(tok.literal, test.expected_literal);
    }
}
