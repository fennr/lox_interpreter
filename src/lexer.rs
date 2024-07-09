/// mcfr4g
use crate::token::{Token, TokenType};

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
        let mut line_number = 1;

        for char in self.source.chars() {
            let token = Lexer::scan_token(char, &mut line_number);

            match token {
                Some(token) => {
                    println!("{} {} {}", token.token_type, token.lexeme, token.literal);
                    self.tokens.push(token);
                },
                None => {
                    self.error_code = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", line_number, char);
                },
            }
        }
        println!("EOF  null");
    }

    fn scan_token(ch: char, line_number: &mut u64) -> Option<Token> {
        let token = match ch {
            '(' => Some(Token::new(TokenType::LEFT_PAREN, ch.to_string(), "null".to_string(), *line_number)),
            ')' => Some(Token::new(TokenType::RIGHT_PAREN, ch.to_string(), "null".to_string(), *line_number)),
            '{' => Some(Token::new(TokenType::LEFT_BRACE, ch.to_string(), "null".to_string(), *line_number)),
            '}' => Some(Token::new(TokenType::RIGHT_BRACE, ch.to_string(), "null".to_string(), *line_number)),
            ',' => Some(Token::new(TokenType::COMMA, ch.to_string(), "null".to_string(), *line_number)),
            '.' => Some(Token::new(TokenType::DOT, ch.to_string(), "null".to_string(), *line_number)),
            '-' => Some(Token::new(TokenType::MINUS, ch.to_string(), "null".to_string(), *line_number)),
            '+' => Some(Token::new(TokenType::PLUS, ch.to_string(), "null".to_string(), *line_number)),
            ';' => Some(Token::new(TokenType::SEMICOLON, ch.to_string(), "null".to_string(), *line_number)),
            '*' => Some(Token::new(TokenType::STAR, ch.to_string(), "null".to_string(), *line_number)),
            '/' => Some(Token::new(TokenType::SLASH, ch.to_string(), "null".to_string(), *line_number)),
            '\n' => {
                *line_number += 1;
                Some(Token::new(TokenType::EOL, TokenType::EOL.to_string(), TokenType::EOL.to_string(), *line_number))
            },
            _ => None,
        };
        token
    }
}

