use crate::ast::{Identifier, LetStatement, Program, Statement, ReturnStatement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::new(TokenType::EOF, String::from("")),
            peek_token: Token::new(TokenType::EOF, String::from("")),
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token_type);
            return false;
        }
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let error = format!("expected next token to be {:?}, got {:?} instead", token_type.as_str(), self.peek_token.token_type.as_str());
        self.errors.push(error);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.token_type != TokenType::EOF {
            match self.parse_statement() {
                Some(statement) => program.statements.push(statement),
                None => {},
            }
            self.next_token();
        }
        return program;
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier{token: self.current_token.clone(), value:self.current_token.literal.clone()};
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: Skipping expressions until we encounter a semicolon
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(LetStatement {
            token,
            name,
            value: Box::new(Identifier {
                token: Token::new(TokenType::Ident, String::from("")),
                value: String::from(""),
            }),
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();
        self.next_token();

        // TODO: Skipping expressions until we encounter a semicolon
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(ReturnStatement {
            token,
            value: Box::new(Identifier {
                token: Token::new(TokenType::Ident, String::from("")),
                value: String::from(""),
            }),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{LetStatement, Node, NodeType, Statement, to_concrete_statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", errors.len());
        for error in errors {
            println!("parser error: {}", error);
        }
        panic!();
    }

    fn test_let_statement(statement: &Box<dyn Statement>, name: &str) -> bool {
        assert_eq!(statement.node_type(), NodeType::LetStatement);
        assert_eq!(statement.token_literal(), String::from("let"));

        let let_statement = to_concrete_statement::<LetStatement>(statement);
        assert_eq!(let_statement.name.value, name);
        assert_eq!(let_statement.name.token_literal(), name);
        return true;
    }

    #[test]
    fn test_let_statements() {
        let input = String::from("let x = 5;
let y = 10;
let foobar = 838383;");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 3);

        let tests = ["x", "y", "foobar"];
        for (i, test) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            assert!(test_let_statement(statement, test));
        }
    }

    #[test]
    fn test_return_statements() {
        let input = String::from("return 5;
return 10;
return 993322;");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 3);

        for statement in program.statements {
            assert_eq!(statement.node_type(), NodeType::ReturnStatement);
            assert_eq!(statement.token_literal(), String::from("return"));
        }
    }
}