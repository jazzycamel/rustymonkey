use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: u64,
    read_position: u64,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u64 {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position as usize] as char
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() as u64 {
            return '\0'
        }
        self.input.as_bytes()[self.read_position as usize] as char
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        self.input[position as usize..self.position as usize].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        self.input[position as usize..self.position as usize].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut token = Token { token_type: TokenType::EOF, literal: String::from(self.ch) };
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    token.token_type = TokenType::EQ;
                    token.literal = ch.to_string() + &self.ch.to_string();
                } else {
                    token.token_type = TokenType::Assign;
                }
            },
            '+' => token.token_type = TokenType::Plus,
            '-' => token.token_type = TokenType::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    token.token_type = TokenType::NotEQ;
                    token.literal = ch.to_string() + &self.ch.to_string();
                } else {
                    token.token_type = TokenType::Bang;
                }
            },
            '*' => token.token_type = TokenType::Asterisk,
            '/' => token.token_type = TokenType::Slash,
            '<' => token.token_type = TokenType::LT,
            '>' => token.token_type = TokenType::GT,
            ';' => token.token_type = TokenType::Semicolon,
            '(' => token.token_type = TokenType::LParen,
            ')' => token.token_type = TokenType::RParen,
            ',' => token.token_type = TokenType::Comma,
            '{' => token.token_type = TokenType::LBrace,
            '}' => token.token_type = TokenType::RBrace,
            '\0' => token.literal = String::from(""),
            _ => {
                if Self::is_letter(self.ch) {
                    token.literal = self.read_identifier();
                    token.token_type = TokenType::lookup_identifier(&token.literal);
                    return token;
                } else if Self::is_digit(self.ch) {
                    token.token_type = TokenType::Int;
                    token.literal = self.read_number();
                    return token;
                } else {
                    token.token_type = TokenType::Illegal;
                }
            }
        }
        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = String::from("let five = 5;
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
10 != 9;");

        struct LexerTest {
            expected_type: TokenType,
            expected_literal: String,
        }

        impl LexerTest {
            fn new(expected_type: TokenType, expected_literal: &str) -> Self {
                Self {
                    expected_type,
                    expected_literal: String::from(expected_literal),
                }
            }
        }

        let tests = [
            LexerTest::new(TokenType::Let, "let"),
            LexerTest::new(TokenType::Ident, "five"),
            LexerTest::new(TokenType::Assign, "="),
            LexerTest::new(TokenType::Int, "5"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::Let, "let"),
            LexerTest::new(TokenType::Ident, "ten"),
            LexerTest::new(TokenType::Assign, "="),
            LexerTest::new(TokenType::Int, "10"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::Let, "let"),
            LexerTest::new(TokenType::Ident, "add"),
            LexerTest::new(TokenType::Assign, "="),
            LexerTest::new(TokenType::Function, "fn"),
            LexerTest::new(TokenType::LParen, "("),
            LexerTest::new(TokenType::Ident, "x"),
            LexerTest::new(TokenType::Comma, ","),
            LexerTest::new(TokenType::Ident, "y"),
            LexerTest::new(TokenType::RParen, ")"),
            LexerTest::new(TokenType::LBrace, "{"),
            LexerTest::new(TokenType::Ident, "x"),
            LexerTest::new(TokenType::Plus, "+"),
            LexerTest::new(TokenType::Ident, "y"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::RBrace, "}"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::Let, "let"),
            LexerTest::new(TokenType::Ident, "result"),
            LexerTest::new(TokenType::Assign, "="),
            LexerTest::new(TokenType::Ident, "add"),
            LexerTest::new(TokenType::LParen, "("),
            LexerTest::new(TokenType::Ident, "five"),
            LexerTest::new(TokenType::Comma, ","),
            LexerTest::new(TokenType::Ident, "ten"),
            LexerTest::new(TokenType::RParen, ")"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::Bang, "!"),
            LexerTest::new(TokenType::Minus, "-"),
            LexerTest::new(TokenType::Slash, "/"),
            LexerTest::new(TokenType::Asterisk, "*"),
            LexerTest::new(TokenType::Int, "5"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::Int, "5"),
            LexerTest::new(TokenType::LT, "<"),
            LexerTest::new(TokenType::Int, "10"),
            LexerTest::new(TokenType::GT, ">"),
            LexerTest::new(TokenType::Int, "5"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::If, "if"),
            LexerTest::new(TokenType::LParen, "("),
            LexerTest::new(TokenType::Int, "5"),
            LexerTest::new(TokenType::LT, "<"),
            LexerTest::new(TokenType::Int, "10"),
            LexerTest::new(TokenType::RParen, ")"),
            LexerTest::new(TokenType::LBrace, "{"),
            LexerTest::new(TokenType::Return, "return"),
            LexerTest::new(TokenType::True, "true"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::RBrace, "}"),
            LexerTest::new(TokenType::Else, "else"),
            LexerTest::new(TokenType::LBrace, "{"),
            LexerTest::new(TokenType::Return, "return"),
            LexerTest::new(TokenType::False, "false"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::RBrace, "}"),
            LexerTest::new(TokenType::Int, "10"),
            LexerTest::new(TokenType::EQ, "=="),
            LexerTest::new(TokenType::Int, "10"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::Int, "10"),
            LexerTest::new(TokenType::NotEQ, "!="),
            LexerTest::new(TokenType::Int, "9"),
            LexerTest::new(TokenType::Semicolon, ";"),
            LexerTest::new(TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for test in tests {
            let token = lexer.next_token();
            assert_eq!(token.token_type, test.expected_type);
            assert_eq!(token.literal, test.expected_literal);
        }
    }
}