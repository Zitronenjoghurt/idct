use egui::Ui;

pub mod curiosity_property;
pub mod data_pack_meta_edit;
pub mod list_edit;
pub mod property_selector;
pub mod tag_rule;
pub mod window_menu;

pub trait Component {
    fn show(self, ui: &mut Ui);
}
