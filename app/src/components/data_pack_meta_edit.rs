use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::data::pack::DataPackMeta;

pub struct DataPackMetaEdit<'a> {
    data_pack_meta: &'a mut DataPackMeta,
}

impl<'a> DataPackMetaEdit<'a> {
    pub fn new(data_pack_meta: &'a mut DataPackMeta) -> Self {
        Self { data_pack_meta }
    }
}

impl Component for DataPackMetaEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("data_pack_meta_edit")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.data_pack_meta.name);
                ui.end_row();
            });
    }
}
