use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("⚠ Invalid character `{character}` at line {line}, column {column}.\n 📖 {text}")]
    InvalidCharacter {
        character: char,
        line: usize,
        column: usize,
        text: String,
    },

    #[error("💥 Unknown error occurred.")]
    Unknown,
}

impl LexerError {
    pub fn invalid_character(character: char, line: usize, column: usize, text: String) -> Self {
        LexerError::InvalidCharacter { character, line, column, text }
    }
}