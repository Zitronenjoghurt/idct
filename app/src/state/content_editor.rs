use idct_game::data::pack::DataPack;
use idct_game::error::GameResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContentEditorState {
    pub edited_pack: DataPack,
}

impl ContentEditorState {
    pub fn restore_core_data(&mut self) -> GameResult<()> {
        self.edited_pack = DataPack::from_core_data()?;
        Ok(())
    }
}
