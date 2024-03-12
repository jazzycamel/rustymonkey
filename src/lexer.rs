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
                    token.token_type = TokenType::ASSIGN;
                }
            },
            '+' => token.token_type = TokenType::PLUS,
            '-' => token.token_type = TokenType::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    token.token_type = TokenType::NOT_EQ;
                    token.literal = ch.to_string() + &self.ch.to_string();
                } else {
                    token.token_type = TokenType::BANG;
                }
            },
            '*' => token.token_type = TokenType::ASTERISK,
            '/' => token.token_type = TokenType::SLASH,
            '<' => token.token_type = TokenType::LT,
            '>' => token.token_type = TokenType::GT,
            ';' => token.token_type = TokenType::SEMICOLON,
            '(' => token.token_type = TokenType::LPAREN,
            ')' => token.token_type = TokenType::RPAREN,
            ',' => token.token_type = TokenType::COMMA,
            '{' => token.token_type = TokenType::LBRACE,
            '}' => token.token_type = TokenType::RBRACE,
            '\0' => token.literal = String::from(""),
            _ => {
                if Self::is_letter(self.ch) {
                    token.literal = self.read_identifier();
                    token.token_type = TokenType::lookup_identifier(&token.literal);
                    return token;
                } else if Self::is_digit(self.ch) {
                    token.token_type = TokenType::INT;
                    token.literal = self.read_number();
                    return token;
                } else {
                    token.token_type = TokenType::ILLEGAL;
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
            LexerTest::new(TokenType::LET, "let"),
            LexerTest::new(TokenType::IDENT, "five"),
            LexerTest::new(TokenType::ASSIGN, "="),
            LexerTest::new(TokenType::INT, "5"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::LET, "let"),
            LexerTest::new(TokenType::IDENT, "ten"),
            LexerTest::new(TokenType::ASSIGN, "="),
            LexerTest::new(TokenType::INT, "10"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::LET, "let"),
            LexerTest::new(TokenType::IDENT, "add"),
            LexerTest::new(TokenType::ASSIGN, "="),
            LexerTest::new(TokenType::FUNCTION, "fn"),
            LexerTest::new(TokenType::LPAREN, "("),
            LexerTest::new(TokenType::IDENT, "x"),
            LexerTest::new(TokenType::COMMA, ","),
            LexerTest::new(TokenType::IDENT, "y"),
            LexerTest::new(TokenType::RPAREN, ")"),
            LexerTest::new(TokenType::LBRACE, "{"),
            LexerTest::new(TokenType::IDENT, "x"),
            LexerTest::new(TokenType::PLUS, "+"),
            LexerTest::new(TokenType::IDENT, "y"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::RBRACE, "}"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::LET, "let"),
            LexerTest::new(TokenType::IDENT, "result"),
            LexerTest::new(TokenType::ASSIGN, "="),
            LexerTest::new(TokenType::IDENT, "add"),
            LexerTest::new(TokenType::LPAREN, "("),
            LexerTest::new(TokenType::IDENT, "five"),
            LexerTest::new(TokenType::COMMA, ","),
            LexerTest::new(TokenType::IDENT, "ten"),
            LexerTest::new(TokenType::RPAREN, ")"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::BANG, "!"),
            LexerTest::new(TokenType::MINUS, "-"),
            LexerTest::new(TokenType::SLASH, "/"),
            LexerTest::new(TokenType::ASTERISK, "*"),
            LexerTest::new(TokenType::INT, "5"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::INT, "5"),
            LexerTest::new(TokenType::LT, "<"),
            LexerTest::new(TokenType::INT, "10"),
            LexerTest::new(TokenType::GT, ">"),
            LexerTest::new(TokenType::INT, "5"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::IF, "if"),
            LexerTest::new(TokenType::LPAREN, "("),
            LexerTest::new(TokenType::INT, "5"),
            LexerTest::new(TokenType::LT, "<"),
            LexerTest::new(TokenType::INT, "10"),
            LexerTest::new(TokenType::RPAREN, ")"),
            LexerTest::new(TokenType::LBRACE, "{"),
            LexerTest::new(TokenType::RETURN, "return"),
            LexerTest::new(TokenType::TRUE, "true"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::RBRACE, "}"),
            LexerTest::new(TokenType::ELSE, "else"),
            LexerTest::new(TokenType::LBRACE, "{"),
            LexerTest::new(TokenType::RETURN, "return"),
            LexerTest::new(TokenType::FALSE, "false"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::RBRACE, "}"),
            LexerTest::new(TokenType::INT, "10"),
            LexerTest::new(TokenType::EQ, "=="),
            LexerTest::new(TokenType::INT, "10"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
            LexerTest::new(TokenType::INT, "10"),
            LexerTest::new(TokenType::NOT_EQ, "!="),
            LexerTest::new(TokenType::INT, "9"),
            LexerTest::new(TokenType::SEMICOLON, ";"),
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