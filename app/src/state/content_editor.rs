use idct_game::curiosity::tag::id::TagID;
use idct_game::data::pack::DataPack;
use idct_game::error::GameResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContentEditorState {
    pub edited_pack: DataPack,
    #[serde(skip, default)]
    pub cached_tag_ids: Vec<TagID>,
}

impl ContentEditorState {
    pub fn update(&mut self) {
        self.update_tag_ids();
    }

    pub fn restore_core_data(&mut self) -> GameResult<()> {
        self.edited_pack = DataPack::from_core_data()?;
        Ok(())
    }

    pub fn update_tag_ids(&mut self) {
        self.cached_tag_ids = self
            .edited_pack
            .data
            .tag_rules
            .rules
            .iter()
            .map(|rule| rule.id.clone())
            .collect();
    }
}
