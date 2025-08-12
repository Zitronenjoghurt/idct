use crate::data::GameData;
use crate::error::GameResult;
use crate::state::GameState;

pub mod curiosity;
pub mod data;
pub mod dimension;
pub mod error;
pub mod state;
mod utils;

pub const CORE_DATA_BYTES: &[u8] = include_bytes!("./../../data/core_data.json");

#[derive(Debug, Default)]
pub struct Game {
    pub data: GameData,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self) -> GameResult<()> {
        self.data.load_from_core_data()?;
        Ok(())
    }
}
