use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("🌋 Already defined function")]
    AlreadyDefinedFunction {
        name: String,
    },

    #[error("💥 Unknown error occurred.")]
    Unknown,
}
