use egui::Ui;

pub mod curiosity_property_definition_edit;
pub mod curiosity_property_definitions_edit;
pub mod data_pack_meta_edit;
pub mod list_edit;
pub mod tag_rule_edit;
pub mod tag_rules_edit;
pub mod window_menu;

pub trait Component {
    fn show(self, ui: &mut Ui);
}
