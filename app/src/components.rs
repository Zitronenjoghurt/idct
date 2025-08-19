use egui::Ui;

pub mod curiosity;
pub mod data_pack_meta_edit;
pub mod dimension;
pub mod list_edit;
pub mod property_selector;
pub mod random_distribution_edit;
pub mod window_menu;

pub trait Component {
    fn show(self, ui: &mut Ui);
}
