#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifiers and literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    EQ,
    NotEQ,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::EOF => "EOF",

            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",

            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",

            TokenType::LT => "<",
            TokenType::GT => ">",

            TokenType::Comma => ",",
            TokenType::Semicolon => ";",

            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",

            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::If => "IF",
            TokenType::Else => "ELSE",
            TokenType::Return => "RETURN",

            TokenType::EQ => "==",
            TokenType::NotEQ => "!=",
        }
    }

    pub fn lookup_identifier(identifier: &String) -> TokenType {
        match identifier.as_str() {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}