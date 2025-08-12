use idct_game::data::GameData;
use idct_game::error::GameResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContentEditorState {
    pub edited_data: GameData,
}

impl ContentEditorState {
    pub fn restore_core_data(&mut self) -> GameResult<()> {
        self.edited_data.clear();
        self.edited_data.load_from_core_data()
    }
}
