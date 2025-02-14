use std::ops::Range;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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

use TokenType::*;
pub const LOGICAL_OPERATORS:        [TokenType; 2] = [LOGICAL_AND, LOGICAL_OR];
pub const ARITHMETIC_OPERATORS:     [TokenType; 4] = [PLUS, MINUS, ASTERISK, SLASH];
pub const ASSIGN_OPERATORS:         [TokenType; 3] = [ASSIGN, PLUS_ASSIGN, MINUS_ASSIGN];
pub const COMPARISSON_OPERATORS:    [TokenType; 6] = [EQ, NEQ, GT, GTE, LT, LTE];


const BINARY_OPERATORS_LENGHT: usize = 
    LOGICAL_OPERATORS.len() + 
    ARITHMETIC_OPERATORS.len() + 
    ASSIGN_OPERATORS.len() + 
    COMPARISSON_OPERATORS.len();

pub const BINARY_OPERATORS:         [TokenType; BINARY_OPERATORS_LENGHT] = [
    LOGICAL_AND, LOGICAL_OR,
    PLUS, MINUS, ASTERISK, SLASH, 
    ASSIGN, PLUS_ASSIGN, MINUS_ASSIGN, 
    EQ, NEQ, GT, GTE, LT, LTE
];

#[derive(Eq, Clone)]
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