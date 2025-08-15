use std::path::Path;
use thiserror::Error;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Unable to read file extension for path: {path}")]
    FileExtensionRead { path: String },
    #[error("Invalid file extension: {extension}")]
    InvalidFileExtension { extension: String },
    #[error("File IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("JSON De/serialization error: {0}")]
    JSONSerialization(#[from] serde_json::Error),
    #[error("RMP Deserialization error: {0}")]
    MPDeserialization(#[from] rmp_serde::decode::Error),
    #[error("RMP Serialization error: {0}")]
    MPSerialization(#[from] rmp_serde::encode::Error),
}

impl GameError {
    pub fn file_extension_read(path: &Path) -> Self {
        Self::FileExtensionRead {
            path: format!("{:?}", path),
        }
    }

    pub fn invalid_file_extension(extension: impl Into<String>) -> Self {
        Self::InvalidFileExtension {
            extension: extension.into(),
        }
    }
}
