use crate::components::window_menu::WindowMenu;
use crate::components::Component;
use crate::state::AppState;
use crate::views::{View, ViewID};
use egui::{Button, Context, MenuBar, RichText, SidePanel, TopBottomPanel};
use serde::{Deserialize, Serialize};

mod data_window;
mod tag_edit_window;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContentEditorView {
    data_window: data_window::DataWindow,
    tag_edit_window: tag_edit_window::TagEditWindow,
}

impl ContentEditorView {
    fn render_windows(&mut self, ctx: &Context, state: &mut AppState) {
        self.data_window.render(ctx, state);
        self.tag_edit_window.render(ctx, state);
    }

    fn on_home_clicked(&self, state: &mut AppState) {
        state.switch_view(ViewID::MainMenu);
    }
}

impl View for ContentEditorView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.render_windows(ctx, state);

        TopBottomPanel::top("content_editor_top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                let home_response = ui.add(Button::new(RichText::new(" 🏠 ").size(20.0)));
                if home_response.clicked() {
                    self.on_home_clicked(state);
                }

                ui.label("Content Editor");

                ui.separator();
            });
        });

        SidePanel::left("content_editor_tag_editor_left_panel").show(ctx, |ui| {
            WindowMenu::new("Windows")
                .window(&mut self.data_window)
                .window(&mut self.tag_edit_window)
                .show(ui);
        });
    }
}
