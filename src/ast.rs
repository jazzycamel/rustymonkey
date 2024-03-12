use as_any::{AsAny};
use crate::token::Token;
use dyn_clone::DynClone;

#[derive(PartialEq, Debug)]
pub enum NodeType {
    Program,
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
    Identifier,
    IntegerLiteral,
    StringLiteral,
    PrefixExpression,
    InfixExpression,
    BooleanLiteral,
    BlockStatement,
    IfExpression,
    FunctionLiteral,
    CallExpression
}

trait Node {
    fn node_type(&self) -> NodeType;
    fn token_literal(&self) -> String;
}

pub trait Statement: DynClone + AsAny {
    fn statement_node(&self);
    fn node_type(&self) -> NodeType;
    fn token_literal(&self) -> String;
}
dyn_clone::clone_trait_object!(Statement);

pub fn to_concrete_statement<T: Clone + 'static>(statement: &Box<dyn Statement>) -> Box<T> {
    let dc = statement.as_ref().as_any().downcast_ref::<T>().unwrap();
    Box::new(dc.clone())
}

pub trait Expression: DynClone + AsAny {
    fn expression_node(&self);
    fn node_type(&self) -> NodeType;
    fn token_literal(&self) -> String;
}
dyn_clone::clone_trait_object!(Expression);

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn node_type(&self) -> NodeType {
        NodeType::Program
    }
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    fn node_type(&self) -> NodeType {
        NodeType::Identifier
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn node_type(&self) -> NodeType {
        NodeType::LetStatement
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn node_type(&self) -> NodeType {
        NodeType::ReturnStatement
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}