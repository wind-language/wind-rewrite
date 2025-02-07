use crate::frontend::token;
use crate::reporter::lex;

pub mod simd;

struct BufferStream {
    src: Vec<char>,
    index: usize,
    position: (usize, usize)
}

impl BufferStream {
    pub fn new(src: String) -> BufferStream {
        BufferStream {
            src: src.chars().collect(),
            index: 0,
            position: (0, 0)
        }
    }

    pub fn advance(&mut self, n: usize) {
        for _ in 0..n {
            if self.src[self.index] == '\n' {
                self.position.0 += 1;
                self.position.1 = 0;
            }
            self.index += 1;
            self.position.1 += 1;
        }
    }
}

pub struct Lexer {
    buffer: BufferStream,
    pub tokens: Vec<token::Token>,
    allow_ws: bool
}


impl Lexer {
    pub fn new(src: String, allow_ws: bool) -> Lexer {
        Lexer {
            buffer: BufferStream::new(src),
            tokens: Vec::new(),
            allow_ws
        }
    }

    fn stream_to_bytes(&self) -> Vec<u8> {
        self.buffer.src.iter().map(|c| *c as u8).collect()
    }

    fn char_err(&mut self, c: char) -> lex::LexerError {
        let mut line: String = "".to_string();
        for i in (0..self.buffer.position.1).rev() {
            if self.buffer.src[i] == '\n' {
                line = self.buffer.src[i..self.buffer.position.1].iter().collect::<String>();
                break;
            }
        }
        line.push(c);
        for i in self.buffer.position.1..self.buffer.src.len() {
            if self.buffer.src[i] == '\n' {
                break;
            }
            line.push(self.buffer.src[i]);
        }
        lex::LexerError::invalid_character(c, self.buffer.position.0+1, self.buffer.position.1, line.to_string())
    }

    fn lex_ws(&mut self) {
        let start = self.buffer.position;
        let mut content = "".to_string();
        while self.buffer.index < self.buffer.src.len() {
            let c = self.buffer.src[self.buffer.index];
            if c == ' ' || c == '\n' || c == '\t' {
                content += &c.to_string();
                self.buffer.advance(1);
            } else {
                break;
            }
        }
        let end = self.buffer.position;
        let position = (
            std::ops::Range { start: start.0, end: end.0 },
            std::ops::Range { start: start.1, end: end.1 },
        );
        self.tokens.push(token::Token::new(token::TokenType::WS, content, position, 0));
    }

    pub fn lex(&mut self) -> Result<(), lex::LexerError> {
        let src_bytes = self.stream_to_bytes();
        while self.buffer.index < src_bytes.len() {
            let c = src_bytes[self.buffer.index] as char;
            match c {
                'a'..='z' | 'A'..='Z' => self.lex_ident(),
                '0'..='9' => self.lex_number(),
                '!' | '=' | '<' | '>' => self.lex_punct(),
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' | ';' => self.lex_punct(),
                '+' | '-' | '*' | '/' => self.lex_punct(),
                ' ' | '\n' | '\t' => {
                    if self.allow_ws { self.lex_ws(); }
                    else { self.buffer.advance(1) }
                },
                _ => {
                    self.buffer.advance(1);
                    return Err(self.char_err(c));
                },
            }
        }
        Ok(())
    }

    pub fn dump_tokens(&self) {
        for token in self.tokens.iter() {
            println!("{:?}", token);
        }
    }
}
