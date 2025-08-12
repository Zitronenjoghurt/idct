use crate::state::AppState;
use crate::views::{View, ViewID};
use crate::windows::settings::SettingsWindow;
use crate::windows::ViewWindow;
use egui::{Button, CentralPanel, Context, MenuBar, RichText, TopBottomPanel, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MainMenuView {
    settings_window: SettingsWindow,
}

impl MainMenuView {
    fn render_windows(&mut self, ctx: &Context, state: &mut AppState) {
        self.settings_window.render(ctx, state);
    }

    fn on_content_editor_clicked(&self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::ContentEditor);
    }

    fn on_debug_clicked(&self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::Debug);
    }
}

impl View for MainMenuView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.render_windows(ctx, state);

        TopBottomPanel::top("main_menu_top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.label("Main Menu");

                ui.separator();

                let mut settings_window_open = self.settings_window.is_open();
                if ui.checkbox(&mut settings_window_open, "Settings").changed() {
                    self.settings_window.set_open(settings_window_open);
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            let vertical_space = ui.available_size_before_wrap().y;
            ui.vertical_centered(|ui| {
                ui.add_space(vertical_space / 5.0);
                ui.heading(RichText::new("INTER-DIMENSIONAL").size(75.0));
                ui.heading(RichText::new("CURIOSITIES TRADER").size(75.0));

                ui.add_space(50.0);
                let content_editor_response = ui.add(
                    Button::new(RichText::new("Content Editor").size(35.0))
                        .min_size(Vec2::new(300.0, 75.0)),
                );

                ui.add_space(25.0);
                let debug_response = ui.add(
                    Button::new(RichText::new("Debug").size(35.0)).min_size(Vec2::new(300.0, 75.0)),
                );

                if content_editor_response.clicked() {
                    self.on_content_editor_clicked(ctx, state);
                }

                if debug_response.clicked() {
                    self.on_debug_clicked(ctx, state);
                }
            });
        });
    }
}
