use core::fmt;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single character tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    

    EOL,
    EOF,
    UNKNOWN,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_val = match self {
            TokenType::LEFT_PAREN => "LEFT_PAREN",
            TokenType::RIGHT_PAREN => "RIGHT_PAREN",
            TokenType::LEFT_BRACE => "LEFT_BRACE",
            TokenType::RIGHT_BRACE => "RIGHT_BRACE",
            TokenType::COMMA => "COMMA",
            TokenType::DOT => "DOT",
            TokenType::MINUS => "MINUS",
            TokenType::PLUS => "PLUS",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::SLASH => "SLASH",
            TokenType::STAR => "STAR",
            TokenType::EOL => "EOL",
            TokenType::EOF => "EOF",
            _ => "UNKNOWN",
        };
        write!(f, "{}", display_val.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u64) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            // "Token {{ token_type: {}, lexeme: {}, literal: {}, line: {} }}",
            // self.token_type, self.lexeme, self.literal, self.line
            "{} {} null",
            self.token_type, self.lexeme
        )
    }
}