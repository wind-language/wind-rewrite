use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("🌋 Already defined function")]
    AlreadyDefinedFunction {
        name: String,
    },

    #[error("🔥 Function `{name}` not found")]
    FunctionNotFound {
        name: String,
    },

    #[error("🚨 Type not found")]
    TypeNotFound {
        name: String,
    },

    #[error("💥 Unknown error occurred.")]
    Unknown,
}
