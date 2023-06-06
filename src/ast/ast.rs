use crate::token::token::*;
#[derive(Debug)]
pub struct Node {}

pub trait NodeTrait {
    fn token_literal(&self) -> String;
}

impl PartialEq<Node> for Node {
    fn eq(&self, other: &Node) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct Statement {
    pub node: Node,
}

#[derive(Debug)]
pub enum StatementType {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    NilStatement,
}

impl PartialEq<StatementType> for StatementType {
    fn eq(&self, other: &StatementType) -> bool {
        match (self, other) {
            (StatementType::NilStatement, StatementType::NilStatement) => true,
            (StatementType::ReturnStatement(ident1), StatementType::ReturnStatement(ident2)) => ident1 == ident2,
            (StatementType::LetStatement(ident1), StatementType::LetStatement(ident2)) => 
                ident1 == ident2,
            
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
}

impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

#[derive(Debug)]
pub struct Expression {
    pub node: Node,
}

trait ExpressionTrait {
    fn expression_node(&mut self);
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
}

impl PartialEq<ReturnStatement> for ReturnStatement {
    fn eq(&self, other: &ReturnStatement) -> bool {
        (self.token == other.token) && (self.value == other.value)
    }
}
