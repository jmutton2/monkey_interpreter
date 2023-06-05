use crate::token::token::*;

pub struct Node {}

trait NodeTrait {
    fn token_literal(&self) -> String;
}

pub struct Statement {
    pub node: Node,
}

trait StatementTrait {
    fn statement_node(&mut self);
}

impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        "test".to_string()
    }
}

struct Expression {
    node: Node,
}

trait ExpressionTrait {
    fn expression_node(&mut self);
}

pub struct Program {
    pub statements: Vec<Statement>,
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

struct Identifier {
    token: Token,
    value: String,
}

impl ExpressionTrait for Identifier {
    fn expression_node(&mut self) {}
}

impl NodeTrait for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.to_owned();
    }
}

struct LetStatement {
    token: Token,
    name: *mut Identifier,
    value: Expression,
}

impl StatementTrait for LetStatement {
    fn statement_node(&mut self) {}
}

impl NodeTrait for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.to_owned();
    }
}
