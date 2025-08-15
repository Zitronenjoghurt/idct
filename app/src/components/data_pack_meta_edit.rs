use crate::components::Component;
use egui::Ui;
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
        ui.text_edit_singleline(&mut self.data_pack_meta.name);
    }
}
