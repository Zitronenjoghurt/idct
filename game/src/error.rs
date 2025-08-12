use thiserror::Error;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("File IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl GameError {
    pub fn io(err: std::io::Error) -> Self {
        Self::IO(err)
    }

    pub fn serialization(err: serde_json::Error) -> Self {
        Self::Serialization(err)
    }
}
