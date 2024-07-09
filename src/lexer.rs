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

    pub fn tokenize(&self) -> i32 {
        let mut out = 0;
        let mut line_number = 1;
        for char in self.source.chars() {
            let status = Lexer::scan_token(char, &mut line_number);
            if status != 0 {
                out = status;
            }
        }
        println!("EOF  null");
        out
    }

    fn scan_token(ch: char, line_number: &mut u64) -> i32 {
        let token = match ch {
            '(' => Token::new(TokenType::LEFT_PAREN, ch.to_string(), ch.to_string(), *line_number),
            ')' => Token::new(TokenType::RIGHT_PAREN, ch.to_string(), ch.to_string(), *line_number),
            '{' => Token::new(TokenType::LEFT_BRACE, ch.to_string(), ch.to_string(), *line_number),
            '}' => Token::new(TokenType::RIGHT_BRACE, ch.to_string(), ch.to_string(), *line_number),
            ',' => Token::new(TokenType::COMMA, ch.to_string(), ch.to_string(), *line_number),
            '.' => Token::new(TokenType::DOT, ch.to_string(), ch.to_string(), *line_number),
            '-' => Token::new(TokenType::MINUS, ch.to_string(), ch.to_string(), *line_number),
            '+' => Token::new(TokenType::PLUS, ch.to_string(), ch.to_string(), *line_number),
            ';' => Token::new(TokenType::SEMICOLON, ch.to_string(), ch.to_string(), *line_number),
            '*' => Token::new(TokenType::STAR, ch.to_string(), ch.to_string(), *line_number),
            '/' => Token::new(TokenType::SLASH, ch.to_string(), ch.to_string(), *line_number),
            '\n' => {
                *line_number += 1;
                Token::new(TokenType::EOL, TokenType::EOL.to_string(), TokenType::EOL.to_string(), *line_number)
            }
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", *line_number, ch.to_string());
                return 65;
            }
        };
        println!("{} {} null", token.token_type, token.lexeme);
        0
    }
}

