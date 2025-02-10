use crate::frontend::lexer;
use crate::frontend::token;
use std::simd::cmp::SimdPartialEq;
use std::simd::{Simd, prelude::SimdPartialOrd};


impl lexer::Lexer {
    pub fn lex_ident(&mut self) {
        let mut literal = String::new();
        let start = (self.buffer.position.0, self.buffer.position.1);
        let src_bytes = self.stream_to_bytes();
    
        while self.buffer.index < src_bytes.len() {
            let remaining = src_bytes.len() - self.buffer.index;
    
            if remaining >= 8 {
                let chunk = Simd::<u8, 8>::from_slice(&src_bytes[self.buffer.index..self.buffer.index + 8]);
    
                let lower_bound_lowercase = Simd::splat(b'a');
                let upper_bound_lowercase = Simd::splat(b'z');
                let lower_bound_uppercase = Simd::splat(b'A');
                let upper_bound_uppercase = Simd::splat(b'Z');
                let lower_bound_numeric = Simd::splat(b'0');
                let upper_bound_numeric = Simd::splat(b'9');
                let lower_bound_extended = Simd::splat(lexer::ASCII_EXTENDED as u8);
                let upper_bound_extended = Simd::splat(lexer::ASCII_EXT_END as u8);
                let underscore = Simd::splat(b'_');
                let colon = Simd::splat(b':');
    
                let is_lower = chunk.simd_ge(lower_bound_lowercase) & chunk.simd_le(upper_bound_lowercase);
                let is_upper = chunk.simd_ge(lower_bound_uppercase) & chunk.simd_le(upper_bound_uppercase);
                let is_numeric = chunk.simd_ge(lower_bound_numeric) & chunk.simd_le(upper_bound_numeric);
                let is_underscore = chunk.simd_eq(underscore);
                let is_double_colon = chunk.simd_eq(colon).to_array();
                let is_extended = chunk.simd_ge(lower_bound_extended) & chunk.simd_le(upper_bound_extended);
    
                let is_valid = is_lower | is_upper | is_numeric | is_underscore | is_extended;
    
                let mut break_index = 8;
                let mut i = 0;
                while i < 8 {
                    if !is_valid.to_array()[i] {
                        if i < 7 && is_double_colon[i] && is_double_colon[i + 1] {
                            literal.push_str("::");
                            i += 2;
                        } else {
                            break_index = i;
                            break;
                        }
                    } else {
                        literal.push(src_bytes[self.buffer.index + i] as char);
                        i += 1;
                    }
                }
                self.buffer.advance(i);
                if break_index < 8 {
                    break;
                }
            } else {
                let c = src_bytes[self.buffer.index] as char;
                let next = if self.buffer.index + 1 < src_bytes.len() {
                    src_bytes[self.buffer.index + 1] as char
                } else {
                    '\0'
                };
    
                if c.is_alphanumeric() || c == '_' || (c == ':' && next == ':') {
                    if c == ':' && next == ':' {
                        literal.push_str("::");
                        self.buffer.advance(2);
                    } else {
                        literal.push(c);
                        self.buffer.advance(1);
                    }
                } else {
                    break;
                }
            }
        }
    
        let end = (self.buffer.position.0, self.buffer.position.1.saturating_sub(1));
        let position = (
            std::ops::Range { start: start.0, end: end.0 },
            std::ops::Range { start: start.1, end: end.1 },
        );
        let token = token::Token::new(token::TokenType::IDENTIFIER, literal, position, 0);
        self.tokens.push(token);
    }
    

    pub fn lex_number(&mut self) {
        let mut literal = String::new();
        let start = (self.buffer.position.0, self.buffer.position.1);
        let src_bytes = self.stream_to_bytes();

        if src_bytes[self.buffer.index] == b'0' && src_bytes[self.buffer.index + 1] == b'x' {
            literal.push('0');
            literal.push('x');
            self.buffer.advance(2);
        }
        if src_bytes[self.buffer.index] == b'0' && src_bytes[self.buffer.index + 1] == b'b' {
            literal.push('0');
            literal.push('b');
            self.buffer.advance(2);
        }
        
        while self.buffer.index < src_bytes.len() {
            let remaining: usize = src_bytes.len() - self.buffer.index;
            if remaining >= 8 {
                let chunk = Simd::<u8, 8>::from_slice(&src_bytes[self.buffer.index..self.buffer.index+8]);
                let lower_bound = Simd::splat(b'0');
                let upper_bound = Simd::splat(b'9');
                let hex_lower_bound = Simd::splat(b'a');
                let hex_upper_bound = Simd::splat(b'f');
                
                let is_decimal = chunk.simd_ge(lower_bound) & chunk.simd_le(upper_bound);
                let is_hex = chunk.simd_ge(hex_lower_bound) & chunk.simd_le(hex_upper_bound);

                let is_number: std::simd::Mask<i8, 8> = is_decimal | is_hex;

                let mut break_index = 8;
                for i in 0..8 {
                    if !is_number.to_array()[i] {
                        break_index = i;
                        break;
                    }
                }
                
                for i in 0..break_index {
                    literal.push(src_bytes[self.buffer.index + i] as char);
                }
                self.buffer.advance(break_index);
                if break_index < 8 {
                    break;
                }
            } else {
                let c = src_bytes[self.buffer.index] as char;
                if c.is_ascii_hexdigit() {
                    literal.push(c);
                    self.buffer.advance(1);
                } else {
                    break;
                }
            }
        }
        let end = (self.buffer.position.0, self.buffer.position.1.saturating_sub(1));
        let position = (
            std::ops::Range { start: start.0, end: end.0 },
            std::ops::Range { start: start.1, end: end.1 },
        );
        let token = token::Token::new(token::TokenType::INTEGER, literal, position, 0);
        self.tokens.push(token);
    }

    pub fn off_range(&self, start: (usize, usize), off: usize) -> (std::ops::Range<usize>, std::ops::Range<usize>) {
        let end = (self.buffer.position.0, self.buffer.position.1.saturating_add(off));
        (
            std::ops::Range { start: start.0, end: end.0 },
            std::ops::Range { start: start.1, end: end.1 },
        )
    }

    pub fn lex_punct(&mut self) {
        let start = (self.buffer.position.0, self.buffer.position.1);
        let src_bytes = self.stream_to_bytes();
        let c = src_bytes[self.buffer.index] as char;
        let next = if self.buffer.index + 1 < src_bytes.len() {
            src_bytes[self.buffer.index + 1] as char
        } else {
            '\0'
        };

        match c {
            '+' => {
                if next == '=' {
                    let token = token::Token::new(token::TokenType::PLUS_ASSIGN, "+=".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                }
                else {
                    let token = token::Token::new(token::TokenType::PLUS, "+".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '-' => {
                if next == '=' {
                    let token = token::Token::new(token::TokenType::MINUS_ASSIGN, "-=".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                }
                else {
                    let token = token::Token::new(token::TokenType::MINUS, "-".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '*' => {
                let token = token::Token::new(token::TokenType::ASTERISK, "*".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            '/' => {
                let token = token::Token::new(token::TokenType::SLASH, "/".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            '=' => {
                if next == '=' {
                    let token = token::Token::new(token::TokenType::EQ, "==".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                } 
                else if next == '>' {
                    let token = token::Token::new(token::TokenType::ARROW, "=>".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                }
                else {
                    let token = token::Token::new(token::TokenType::ASSIGN, "=".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '!' => {
                if next == '=' {
                    let token = token::Token::new(token::TokenType::NEQ, "!=".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                } else {
                    let token = token::Token::new(token::TokenType::NOT, "!".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '<' => {
                if next == '=' {
                    let token = token::Token::new(token::TokenType::LTE, "<=".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                } else {
                    let token = token::Token::new(token::TokenType::LT, "<".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '>' => {
                if next == '=' {
                    let token = token::Token::new(token::TokenType::GTE, ">=".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                } else {
                    let token = token::Token::new(token::TokenType::GT, ">".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '(' => {
                let token = token::Token::new(token::TokenType::LPAREN, "(".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            ')' => {
                let token = token::Token::new(token::TokenType::RPAREN, ")".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            '{' => {
                let token = token::Token::new(token::TokenType::LBRACE, "{".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            '}' => {
                let token = token::Token::new(token::TokenType::RBRACE, "}".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            '[' => {
                let token = token::Token::new(token::TokenType::LBRACKET, "[".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            ']' => {
                let token = token::Token::new(token::TokenType::RBRACKET, "]".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            ',' => {
                let token = token::Token::new(token::TokenType::COMMA, ",".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            ':' => {
                let token = token::Token::new(token::TokenType::COLON, ":".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            ';' => {
                let token = token::Token::new(token::TokenType::SEMICOLON, ";".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            '&' => {
                if next == '&' {
                    let token = token::Token::new(token::TokenType::LOGICAL_AND, "&&".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                } else {
                    let token = token::Token::new(token::TokenType::AND, "&".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '|' => {
                if next == '|' {
                    let token = token::Token::new(token::TokenType::LOGICAL_OR, "||".to_string(), self.off_range(start, 1), 0);
                    self.tokens.push(token);
                    self.buffer.advance(2);
                } else {
                    let token = token::Token::new(token::TokenType::OR, "|".to_string(), self.off_range(start, 0), 0);
                    self.tokens.push(token);
                    self.buffer.advance(1);
                }
            },
            '.' => {
                let token = token::Token::new(token::TokenType::DOT, ".".to_string(), self.off_range(start, 0), 0);
                self.tokens.push(token);
                self.buffer.advance(1);
            },
            _ => {
                self.buffer.advance(1);
            }
        }
    }
}