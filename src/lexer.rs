use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&self) {
        let mut line_number = 1;
        for char in self.source.chars() {
            line_number = Lexer::scan_token(char, line_number);
        }
        println!("EOF  null");
    }

    fn scan_token(ch: char, mut line_number: u64) -> u64 {
        let token = match ch {
            '(' => Token::new(TokenType::LEFT_PAREN, ch.to_string(), ch.to_string(), line_number),
            ')' => Token::new(TokenType::RIGHT_PAREN, ch.to_string(), ch.to_string(), line_number),
            '\n' => {
                line_number += 1;
                Token::new(TokenType::EOL, TokenType::EOL.to_string(), TokenType::EOL.to_string(), line_number)
            }
            _ => Token::new(TokenType::UNKNOWN, ch.to_string(), ch.to_string(), line_number),
        };
        println!("{}", token);
        line_number
    }
}

