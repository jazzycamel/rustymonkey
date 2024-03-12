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
    CallExpression,
}

macro_rules! node_type_fn {
    ($node_type:expr) => {
        fn node_type(&self) -> NodeType {
            $node_type
        }
    };
}

macro_rules! token_literal_fn {
    () => {
        fn token_literal(&self) -> String {
            self.token.literal.clone()
        }
    };
}

macro_rules! impl_node {
    ($T:ident,$node_type:expr) => {
        impl Node for $T {
            node_type_fn!($node_type);
            token_literal_fn!();
        }
    };
}

pub trait Node {
    fn node_type(&self) -> NodeType;
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + DynClone + AsAny {
    fn statement_node(&self) {}
}
dyn_clone::clone_trait_object!(Statement);

pub fn to_concrete_statement<T: Clone + 'static>(statement: &Box<dyn Statement>) -> Box<T> {
    let dc = statement.as_ref().as_any().downcast_ref::<T>().unwrap();
    Box::new(dc.clone())
}

pub trait Expression: Node + DynClone + AsAny {
    fn expression_node(&self) {}
}
dyn_clone::clone_trait_object!(Expression);

pub fn to_concrete_expression<T: Clone + 'static>(expression: &Box<dyn Expression>) -> Box<T> {
    let dc = expression.as_ref().as_any().downcast_ref::<T>().unwrap();
    Box::new(dc.clone())
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    node_type_fn!(NodeType::Program);

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

impl_node!(Identifier, NodeType::Identifier);

impl Expression for Identifier {}

#[derive(Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl_node!(LetStatement, NodeType::LetStatement);

impl Statement for LetStatement {}

#[derive(Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<dyn Expression>,
}

impl_node!(ReturnStatement, NodeType::ReturnStatement);

impl Statement for ReturnStatement {}

#[derive(Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl_node!(ExpressionStatement, NodeType::ExpressionStatement);

impl Statement for ExpressionStatement {}