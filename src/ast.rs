use std::fmt::{Display, Formatter, Result};
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

pub trait Node: Display {
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

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut out = String::new();
        for statement in &self.statements {
            out.push_str(&statement.to_string());
        }
        write!(f, "{}", out)
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

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl_node!(LetStatement, NodeType::LetStatement);

impl Statement for LetStatement {}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} = {};", self.token_literal(), self.name, self.value)
    }
}

#[derive(Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Box<dyn Expression>>,
}

impl_node!(ReturnStatement, NodeType::ReturnStatement);

impl Statement for ReturnStatement {}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(
            match &self.value {
                Some(value) => format!(" {}", value),
                None => String::from(""),
            }
                .as_str(),
        );
        out.push_str(";");
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl_node!(IntegerLiteral, NodeType::IntegerLiteral);

impl Expression for IntegerLiteral {}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.token.literal)
    }
}

#[derive(Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl_node!(ExpressionStatement, NodeType::ExpressionStatement);

impl Statement for ExpressionStatement {}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.expression.is_none() {
            write!(f, "")
        } else {
            write!(f, "{}", self.expression.as_ref().unwrap())
        }
    }
}

#[derive(Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl_node!(PrefixExpression, NodeType::PrefixExpression);

impl Expression for PrefixExpression {}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl_node!(InfixExpression, NodeType::InfixExpression);

impl Expression for InfixExpression {}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Identifier, LetStatement, Program};
    use crate::token::{Token, TokenType};

    #[test]
    fn test_string() {
        let mut program = Program::new();

        let token = Token::new(TokenType::Let, "let".to_string());
        let name = Identifier {
            token: Token::new(TokenType::Ident, "myVar".to_string()),
            value: "myVar".to_string(),
        };
        let value = Box::new(Identifier {
            token: Token::new(TokenType::Ident, "anotherVar".to_string()),
            value: "anotherVar".to_string(),
        });

        program.statements.push(
            Box::new(LetStatement {
                token,
                name,
                value,
            })
        );

        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}