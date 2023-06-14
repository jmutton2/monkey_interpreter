use crate::token::token::*;
#[derive(Debug)]
pub struct Node {}

pub trait NodeTrait {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

impl PartialEq<Node> for Node {
    fn eq(&self, other: &Node) -> bool {
        true
    }
}


#[derive(Debug)]
pub enum StatementType {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement,
    NilStatement,
}

impl PartialEq<StatementType> for StatementType {
    fn eq(&self, other: &StatementType) -> bool {
        match (self, other) {
            (StatementType::NilStatement, StatementType::NilStatement) => true,
            (StatementType::ExpressionStatement, StatementType::ExpressionStatement) => true,
            (StatementType::ReturnStatement(ident1), StatementType::ReturnStatement(ident2)) => {
                ident1 == ident2
            }
            (StatementType::LetStatement(ident1), StatementType::LetStatement(ident2)) => {
                ident1 == ident2
            }

            _ => false,
        }
    }
}

trait StatementTrait {
    fn statement_node(&self);
}

impl NodeTrait for StatementType {
    fn token_literal(&self) -> String {
        "let".to_string()
    }

    fn string(&self) -> String {
    
    match self {
            StatementType::NilStatement => StatementType::NilStatement.string(),
           StatementType::ExpressionStatement => StatementType::ExpressionStatement.string(),
            StatementType::ReturnStatement(ident1) => ident1.string(),
            StatementType::LetStatement(ident1) => ident1.string(),
            _ => "idk".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Expression {
    pub node: Node,
}

trait ExpressionTrait {
    fn expression_node(&mut self);
}

impl NodeTrait for Expression {
    fn token_literal(&self) -> String {
        return "expression".to_string();
    }

    fn string(&self) -> String {
        let mut parsed_string = "".to_owned();

        return parsed_string;
    }
}

impl PartialEq<Expression> for Expression {
    fn eq(&self, other: &Expression) -> bool {
        self.node == other.node
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<StatementType>,
}

impl NodeTrait for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "".to_string();
        }
    }

    fn string(&self) -> String {
        let mut parsed_string = "".to_owned();

        for stmt in &self.statements {
            parsed_string.push_str(&stmt.string())
        }

        return parsed_string;
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl ExpressionTrait for Identifier {
    fn expression_node(&mut self) {}
}

impl NodeTrait for Identifier {
    fn token_literal(&self) -> String {
        return self.token.as_literal();
    }

    fn string(&self) -> String {
        return self.value.to_owned();
    }
}

impl PartialEq<Identifier> for Identifier {
    fn eq(&self, other: &Identifier) -> bool {
        (self.token == other.token) && (self.value == other.value)
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl StatementTrait for LetStatement {
    fn statement_node(&self) {}
}

impl NodeTrait for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.as_literal();
    }

    fn string(&self) -> String {
        let mut parsed_string = "".to_owned();

        parsed_string.push_str(&self.token_literal());
        parsed_string.push_str(" ");
        parsed_string.push_str(&self.name.string());
        parsed_string.push_str(" = ");
        parsed_string.push_str(";");

        return parsed_string;
    }
}

impl PartialEq<LetStatement> for LetStatement {
    fn eq(&self, other: &LetStatement) -> bool {
        (self.token == other.token) && (self.name == other.name) && (self.value == other.value)
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

impl StatementTrait for ReturnStatement {
    fn statement_node(&self) {}
}

impl NodeTrait for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.as_literal();
    }

    fn string(&self) -> String {
        let mut parsed_string = "".to_owned();

        parsed_string.push_str(&self.token_literal());
        parsed_string.push_str(&self.value.string());
        parsed_string.push_str(";");

        return parsed_string;
    }
}

impl PartialEq<ReturnStatement> for ReturnStatement {
    fn eq(&self, other: &ReturnStatement) -> bool {
        (self.token == other.token) && (self.value == other.value)
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub value: Expression,
}

impl StatementTrait for ExpressionStatement {
    fn statement_node(&self) {}
}

impl NodeTrait for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.as_literal();
    }

    fn string(&self) -> String {
        let parsed_string = "".to_owned();

        return parsed_string;
    }
}

impl PartialEq<ExpressionStatement> for ExpressionStatement {
    fn eq(&self, other: &ExpressionStatement) -> bool {
        (self.token == other.token) && (self.value == other.value)
    }
}

#[test]
fn test_string() {
    let program = Program {
        statements: vec![StatementType::LetStatement(LetStatement {
            token: Token::Let,
            name: Identifier {
                token: Token::Ident("myVar".to_string()),
                value: "myVar".to_string(),
            },
            value: Expression { node: Node {} },
        })],
    };

    assert_eq!(program.string(), "let myVar = ;".to_string());
}
