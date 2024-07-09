/// mcfr4g
use crate::token::{Token, TokenType};
use core::hash;
use std::clone::{self, Clone};
use std::{arch::x86_64::_SIDD_LEAST_SIGNIFICANT, collections::binary_heap::Iter, iter::Peekable};

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    pub error_code: u8,
    pub error_text: String,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: Vec::new(),
            error_code: 0,
            error_text: String::new(),
        }
    }

    pub fn tokenize(&mut self) {
        let binding = self.source.clone();
        for (line_index, line) in binding.lines().enumerate() {
            let mut char_iter = line.chars().peekable();
            while let Some(current_char) = char_iter.next() {
                if current_char.is_whitespace() {
                    continue;
                }
                if let Some(token) = self.scan_token(line_index + 1, current_char, &mut char_iter) {
                    if token.token_type == TokenType::COMMENT {
                        break;
                    }
                    println!("{:?} {} {}", token.token_type, token.lexeme, token.literal);
                } else {
                    eprintln!("[line {}] Error: {}", line_index + 1, self.error_text);
                }
            }
        }
        println!("EOF  null");
    }

    fn scan_token<I>(&mut self, line: usize, ch: char, iter: &mut Peekable<I>) -> Option<Token>
    where
        I: Iterator<Item = char> + Clone,
    {
        let token = match ch {
            '(' => Some(Token::new(
                TokenType::LEFT_PAREN,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            ')' => Some(Token::new(
                TokenType::RIGHT_PAREN,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            '{' => Some(Token::new(
                TokenType::LEFT_BRACE,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            '}' => Some(Token::new(
                TokenType::RIGHT_BRACE,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            ',' => Some(Token::new(
                TokenType::COMMA,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            '.' => Some(Token::new(
                TokenType::DOT,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            '-' => Some(Token::new(
                TokenType::MINUS,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            '+' => Some(Token::new(
                TokenType::PLUS,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            ';' => Some(Token::new(
                TokenType::SEMICOLON,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            '*' => Some(Token::new(
                TokenType::STAR,
                ch.to_string(),
                "null".to_string(),
                line,
            )),
            '!' | '=' | '<' | '>' | '/' => self.scan_comparison_operator(line, ch, iter),
            '"' => self.scan_string(line, iter),
            '0'..='9' => self.scan_number(line, ch, iter),
            _ => {
                self.error_text = format!("Unexpected character: {}", ch);
                self.error_code = 65;
                None
            }
        };
        token
    }

    fn scan_comparison_operator<I>(
        &mut self,
        line: usize,
        ch: char,
        iter: &mut Peekable<I>,
    ) -> Option<Token>
    where
        I: Iterator<Item = char>,
    {
        let next_char = iter.peek();
        let (token_type, lexeme) = match (ch, next_char) {
            ('=', Some('=')) => (TokenType::EQUAL_EQUAL, "==".to_string()),
            ('!', Some('=')) => (TokenType::BANG_EQUAL, "!=".to_string()),
            ('<', Some('=')) => (TokenType::LESS_EQUAL, "<=".to_string()),
            ('>', Some('=')) => (TokenType::GREATER_EQUAL, ">=".to_string()),
            ('/', Some('/')) => (TokenType::COMMENT, "//".to_string()),
            ('=', _) => (TokenType::EQUAL, ch.to_string()),
            ('!', _) => (TokenType::BANG, ch.to_string()),
            ('<', _) => (TokenType::LESS, ch.to_string()),
            ('>', _) => (TokenType::GREATER, ch.to_string()),
            ('/', _) => (TokenType::SLASH, ch.to_string()),
            _ => unreachable!(),
        };
        if lexeme.len() > 1 {
            iter.next();
        }
        Some(Token::new(token_type, lexeme, "null".to_string(), line))
    }

    fn scan_string<I>(&mut self, line: usize, iter: &mut Peekable<I>) -> Option<Token>
    where
        I: Iterator<Item = char>,
    {
        let mut string = String::new();
        let mut last_char = '"';
        while let Some(current_char) = iter.next() {
            last_char = current_char;
            if current_char == '"' {
                break;
            }
            string.push(current_char);
        }
        if last_char != '"' {
            self.error_code = 65;
            self.error_text = "Unterminated string.".to_string();
            None
        } else {
            Some(Token::new(
                TokenType::STRING,
                format!("\"{}\"", string),
                string,
                line,
            ))
        }
    }

    fn scan_number<I>(&mut self, line: usize, ch: char, iter: &mut Peekable<I>) -> Option<Token>
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut clone_iter = iter.clone();
        let mut string = String::new();
        let mut has_dot = false;
        string.push(ch);
        while let Some(current_char) = clone_iter.peek() {
            match current_char {
                '0'..='9' => string.push(*current_char),
                '.' => {
                    if has_dot {
                        break;
                    }
                    has_dot = true;
                    string.push(*current_char);
                }
                _ => break,
            }
            clone_iter.next();
        }
        let mut literal = string.clone();
        if string.ends_with('.') {
            string.pop();
            literal.push('0');
        }
        for _ in 0..string.len() - 1 {
            iter.next();
        }
        if !has_dot {
            literal.push_str(&".0");
        }
        Some(Token::new(TokenType::NUMBER, string, literal, line))
    }
}
