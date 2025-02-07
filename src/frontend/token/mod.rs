use std::ops::Range;

#[derive(PartialEq, Eq, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    IDENTIFIER,
    INTEGER,
    FLOAT,
    STRING,
    CHAR,

    FN,
    LET,
    CONST,
    IF,
    ELSE,
    WHILE,
    RETURN,

    PLUS,
    MINUS,
    ASTERISK,
    SLASH,

    EQ,
    NEQ,
    LT,
    GT,
    LTE,
    GTE,

    NOT,
    AND,
    OR,

    LOGICAL_AND,
    LOGICAL_OR,

    ASSIGN,
    PLUS_ASSIGN,
    MINUS_ASSIGN,

    DOT,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    COMMA,
    COLON,
    SEMICOLON,

    ARROW,

    WS
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub position: (Range<usize>, Range<usize>),
    pub src: usize
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
            "[{:?}]: {:?} at {:?}", self.token_type, self.literal, self.position
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, position: (Range<usize>, Range<usize>), src: usize) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
            position: position,
            src: src
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type && self.literal == other.literal
    }
}