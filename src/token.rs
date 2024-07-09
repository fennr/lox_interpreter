use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single character tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    COMMENT,
    
    // Literals
    IDENTIFIER, STRING, NUMBER,
    
    // Keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

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

            TokenType::BANG => "BANG",
            TokenType::BANG_EQUAL => "BANG_EQUAL",
            TokenType::EQUAL => "EQUAL",
            TokenType::EQUAL_EQUAL => "EQUAL_EQUAL",
            TokenType::GREATER => "GREATER",
            TokenType::GREATER_EQUAL => "GREATER_EQUAL",
            TokenType::LESS => "LESS",
            TokenType::LESS_EQUAL => "LESS_EQUAL",
            TokenType::COMMENT => "COMMENT",

            TokenType::EOF => "EOF",
            _ => "UNKNOWN",
        };
        write!(f, "{}", display_val.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Token {
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