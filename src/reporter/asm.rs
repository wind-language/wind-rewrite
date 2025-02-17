use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssemblerError {
    #[error("🌋 Cannot resolve jump to {symbol}")]
    CannotResolveJump {
        symbol: String,
    },

    #[error("💥 Unknown error occurred.")]
    Unknown,
}
