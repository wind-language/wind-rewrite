use thiserror::Error;

#[derive(Debug, Error)]
pub enum PreprocessorError {
    #[error("📂 Failed to read file `{filename}`:\n  🛑 {source}")]
    FileReadError {
        filename: String,
        #[source]
        source: std::io::Error,
    },

    #[error("⚠ Missing parenthesis `(` in macro call.\n 📖 {text}")]
    MissingParenthesis {
        text: String,
    },

    #[error("💥 Unknown error occurred.")]
    Unknown,
}

impl PreprocessorError {
    pub fn file_read_error(filename: String, source: std::io::Error) -> Self {
        PreprocessorError::FileReadError { filename, source }
    }

    pub fn missing_parenthesis(text: String) -> Self {
        PreprocessorError::MissingParenthesis { text }
    }
}