use crate::ast::{Identifier, LetStatement, Program, Statement, ReturnStatement, Expression,
                 ExpressionStatement, IntegerLiteral, PrefixExpression, InfixExpression};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

fn precedence_for_token_type(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::EQ => Precedence::Equals,
        TokenType::NotEQ => Precedence::Equals,
        TokenType::LT => Precedence::LessGreater,
        TokenType::GT => Precedence::LessGreater,
        TokenType::Plus => Precedence::Sum,
        TokenType::Minus => Precedence::Sum,
        TokenType::Slash => Precedence::Product,
        TokenType::Asterisk => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        _ => Precedence::Lowest,
    }
}

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
        let error = format!("expected next token to be {:?}, got {:?} instead",
                            token_type.as_str(),
                            self.peek_token.token_type.as_str());
        self.errors.push(error);
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_for_token_type(&self.peek_token.token_type)
    }

    fn current_precedence(&self) -> Precedence {
        precedence_for_token_type(&self.current_token.token_type)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.token_type != TokenType::EOF {
            match self.parse_statement() {
                Some(statement) => program.statements.push(statement),
                None => {}
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
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone()
        };
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
            value: Some(Box::new(Identifier {
                token: Token::new(TokenType::Ident, String::from("")),
                value: String::from(""),
            })),
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();
        let expression = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(ExpressionStatement {
            token,
            expression,
        }))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let mut left = match self.current_token.token_type {
            TokenType::Ident => self.parse_identifier(),
            TokenType::Int => self.parser_integer_literal(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            _ => {
                self.errors.push(
                    format!("no prefix parse function for {:?} found",
                            self.current_token.token_type));
                return None;
            }
        };

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            left = match self.peek_token.token_type {
                TokenType::Plus | TokenType::Minus | TokenType::Slash | TokenType::Asterisk |
                TokenType::EQ | TokenType::NotEQ | TokenType::LT | TokenType::GT => {
                    self.next_token();
                    self.parse_infix_expression(left.unwrap())
                }
                _ => return left,
            };
        }
        left
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    fn parser_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.current_token.clone();
        match self.current_token.literal.parse::<i64>() {
            Ok(value) => Some(Box::new(IntegerLiteral {
                token,
                value,
            })),
            Err(_) => {
                self.errors.push(
                    format!("could not parse {:?} as integer",
                            self.current_token.literal));
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix).unwrap();
        Some(Box::new(PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence).unwrap();
        Some(Box::new(InfixExpression {
            token,
            operator,
            left,
            right,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ExpressionStatement, IntegerLiteral, LetStatement, Node, NodeType, Statement,
                     to_concrete_expression, to_concrete_statement, PrefixExpression, Expression,
                     InfixExpression};
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

    fn test_integer_literal(expression: &Box<dyn Expression>, value: i64) -> bool {
        assert_eq!(expression.node_type(), NodeType::IntegerLiteral);
        let integer_literal = to_concrete_expression::<IntegerLiteral>(expression);
        assert_eq!(integer_literal.value, value);
        assert_eq!(integer_literal.token_literal(), value.to_string());
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

    #[test]
    fn test_identifier_expression() {
        let input = String::from("foobar;");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert_eq!(statement.node_type(), NodeType::ExpressionStatement);
        let expression = to_concrete_statement::<ExpressionStatement>(statement).expression.unwrap();
        assert_eq!(expression.node_type(), NodeType::Identifier);
        assert_eq!(expression.token_literal(), String::from("foobar"));
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = String::from("5;");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert_eq!(statement.node_type(), NodeType::ExpressionStatement);
        let expression_statement = to_concrete_statement::<ExpressionStatement>(statement);

        assert!(!expression_statement.expression.is_none());
        let expression = expression_statement.expression.unwrap();

        assert_eq!(expression.node_type(), NodeType::IntegerLiteral);
        let integer_literal = to_concrete_expression::<IntegerLiteral>(&expression);
        assert_eq!(integer_literal.value, 5);
        assert_eq!(integer_literal.token_literal(), String::from("5"));
    }

    macro_rules! test_prefix_expression {
        ($($name:ident: $value:expr)*) => {
        $(
            #[test]
            fn $name(){
                let (input, operator, value) = $value;

                let lexer = Lexer::new(String::from(input));
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                check_parser_errors(&parser);
                assert_eq!(program.statements.len(), 1);

                let statement = &program.statements[0];
                assert_eq!(statement.node_type(), NodeType::ExpressionStatement);
                let expression_statement = to_concrete_statement::<ExpressionStatement>(statement);

                assert!(!expression_statement.expression.is_none());
                let expression = expression_statement.expression.unwrap();

                assert_eq!(expression.node_type(), NodeType::PrefixExpression);
                let prefix_expression = to_concrete_expression::<PrefixExpression>(&expression);

                assert_eq!(prefix_expression.operator, operator);
                assert!(test_integer_literal(&prefix_expression.right, value));
            }
        )*
        }
    }

    test_prefix_expression! {
        test_prefix_expression_1: ("!5;", "!", 5)
        test_prefix_expression_2: ("-15;", "-", 15)
    }

    macro_rules! test_infix_expression {
        ($($name:ident: $value:expr)*) => {
        $(
            #[test]
            fn $name(){
                let (input, left_value, operator, right_value) = $value;

                let lexer = Lexer::new(String::from(input));
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                check_parser_errors(&parser);
                assert_eq!(program.statements.len(), 1);

                let statement = &program.statements[0];
                assert_eq!(statement.node_type(), NodeType::ExpressionStatement);
                let expression_statement = to_concrete_statement::<ExpressionStatement>(statement);

                assert!(!expression_statement.expression.is_none());
                let expression = expression_statement.expression.unwrap();

                assert_eq!(expression.node_type(), NodeType::InfixExpression);
                let infix_expression = to_concrete_expression::<InfixExpression>(&expression);

                assert!(test_integer_literal(&infix_expression.left, left_value));
                assert_eq!(infix_expression.operator, operator);
                assert!(test_integer_literal(&infix_expression.right, right_value));
            }
        )*
        }
    }
    test_infix_expression! {
        test_infix_expression_1: ("5 + 5;", 5, "+", 5)
        test_infix_expression_2: ("5 - 5;", 5, "-", 5)
        test_infix_expression_3: ("5 * 5;", 5, "*", 5)
        test_infix_expression_4: ("5 / 5;", 5, "/", 5)
        test_infix_expression_5: ("5 > 5;", 5, ">", 5)
        test_infix_expression_6: ("5 < 5;", 5, "<", 5)
        test_infix_expression_7: ("5 == 5;", 5, "==", 5)
        test_infix_expression_8: ("5 != 5;", 5, "!=", 5)
    }

    macro_rules! test_operator_precedence_parsing {
        ($($name:ident: $value:expr)*) => {
        $(
            #[test]
            fn $name(){
                let (input, expected) = $value;

                let lexer = Lexer::new(String::from(input));
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                check_parser_errors(&parser);

                assert_eq!(program.to_string(), expected);
            }
        )*
        }
    }

    test_operator_precedence_parsing! {
        test_operator_precedence_parsing_1: ("-a * b","((-a) * b)")
        test_operator_precedence_parsing_2: ("!-a", "(!(-a))")
        test_operator_precedence_parsing_3: ("a + b + c", "((a + b) + c)")
        test_operator_precedence_parsing_4: ("a + b - c", "((a + b) - c)")
        test_operator_precedence_parsing_5: ("a * b * c", "((a * b) * c)")
        test_operator_precedence_parsing_6: ("a * b / c", "((a * b) / c)")
        test_operator_precedence_parsing_7: ("a + b / c", "(a + (b / c))")
        test_operator_precedence_parsing_8: ("a + b * c + d / e - f",
            "(((a + (b * c)) + (d / e)) - f)")
        test_operator_precedence_parsing_9: ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)")
        test_operator_precedence_parsing_10: ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))")
        test_operator_precedence_parsing_11: ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))")
        test_operator_precedence_parsing_12: ("3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))")
    }
}