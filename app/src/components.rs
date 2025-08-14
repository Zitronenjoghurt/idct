use egui::Ui;

pub mod tag_rule_edit;
pub mod tag_rules_edit;
pub mod window_menu;

pub trait Component {
    fn show(self, ui: &mut Ui);
}
