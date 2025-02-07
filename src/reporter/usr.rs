use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("❌ No source file provided.\n ℹ️  Usage: {usage}")]
    NoSourceFileProvided { usage: String },

    #[error("📂 Failed to read file `{filename}`:\n  🛑 {source}")]
    FileReadError {
        filename: String,
        #[source]
        source: std::io::Error,
    },

    #[error("💥 Unknown error occurred.")]
    Unknown,
}


impl CliError {
    pub fn missing_file_usage(program_name: String) -> Self {
        CliError::NoSourceFileProvided {
            usage: format!("{} <source file>\n  ℹ️  Use `--help` for more information.", program_name)
        }
    }

    pub fn file_read_error(filename: String, source: std::io::Error) -> Self {
        CliError::FileReadError { filename, source }
    }
}