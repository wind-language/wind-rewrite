use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectError {
    #[error("💥 Symbol not found {symbol}")]
    SymbolNotFound { symbol: String },

    #[error("💥 Failed relocation of symbol {symbol}")]
    RelocationFailed { symbol: String },

    #[error("💥 File writing failed {file}")]
    FileWriteFailed { file: String },

    #[error("💥 Unknown error occurred.")]
    Unknown,
}

