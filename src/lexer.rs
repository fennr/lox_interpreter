/// mcfr4g
use crate::token::{Token, TokenType};
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    pub error_code: u8,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: Vec::new(),
            error_code: 0,
        }
    }

    pub fn tokenize(&mut self) {
        let binding = self.source.clone();
        for (i, line) in binding.split('\n').enumerate() {
            let mut line_iter = line.chars().peekable();
            while let Some(char) = line_iter.next() {
                if let Some(t) = self.scan_token(i+1, char, &mut line_iter) {
                    println!("{} {} {}", t.token_type, t.lexeme, t.literal);
                } else {
                    self.error_code = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", i+1, char);
                }
            }
        }
        println!("EOF  null");
    }

    fn scan_token<I>(&mut self, line: usize, ch: char, iter: &mut Peekable<I>) -> Option<Token> 
    where I: Iterator<Item = char> {
        let token = match ch {
            '(' => Some(Token::new(TokenType::LEFT_PAREN, ch.to_string(), "null".to_string(), line)),
            ')' => Some(Token::new(TokenType::RIGHT_PAREN, ch.to_string(), "null".to_string(), line)),
            '{' => Some(Token::new(TokenType::LEFT_BRACE, ch.to_string(), "null".to_string(), line)),
            '}' => Some(Token::new(TokenType::RIGHT_BRACE, ch.to_string(), "null".to_string(), line)),
            ',' => Some(Token::new(TokenType::COMMA, ch.to_string(), "null".to_string(), line)),
            '.' => Some(Token::new(TokenType::DOT, ch.to_string(), "null".to_string(), line)),
            '-' => Some(Token::new(TokenType::MINUS, ch.to_string(), "null".to_string(), line)),
            '+' => Some(Token::new(TokenType::PLUS, ch.to_string(), "null".to_string(), line)),
            ';' => Some(Token::new(TokenType::SEMICOLON, ch.to_string(), "null".to_string(), line)),
            '*' => Some(Token::new(TokenType::STAR, ch.to_string(), "null".to_string(), line)),
            '/' => Some(Token::new(TokenType::SLASH, ch.to_string(), "null".to_string(), line)),
            '!' => if let Some('=') = iter.peek() {
                iter.next();
                Some(Token::new(TokenType::BANG_EQUAL, "!=".to_string(), "null".to_string(), line))
            } else {
                Some(Token::new(TokenType::BANG, ch.to_string(), "null".to_string(), line))
            },
            '=' => if let Some('=') = iter.peek() {
                iter.next();
                Some(Token::new(TokenType::EQUAL_EQUAL, "==".to_string(), "null".to_string(), line))
            } else {
                Some(Token::new(TokenType::EQUAL, ch.to_string(), "null".to_string(), line))
            },
            '<' => if let Some('=') = iter.peek() {
                iter.next();
                Some(Token::new(TokenType::LESS_EQUAL, "<=".to_string(), "null".to_string(), line))
            } else {
                Some(Token::new(TokenType::LESS, ch.to_string(), "null".to_string(), line))
            },
            '>' => if let Some('=') = iter.peek() {
                iter.next();
                Some(Token::new(TokenType::GREATER_EQUAL, "!=".to_string(), "null".to_string(), line))
            } else {
                Some(Token::new(TokenType::GREATER, ch.to_string(), "null".to_string(), line))
            },
            '\n' => Some(Token::new(TokenType::EOL, TokenType::EOL.to_string(), TokenType::EOL.to_string(), line)),
            _ => None,
        };
        token
    }
}

