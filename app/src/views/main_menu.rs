use crate::state::AppState;
use crate::views::{View, ViewID};
use egui::{Button, CentralPanel, Context, RichText, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MainMenuView;

impl MainMenuView {
    pub fn on_debug_clicked(&self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::Debug);
    }
}

impl View for MainMenuView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        CentralPanel::default().show(ctx, |ui| {
            let vertical_space = ui.available_size_before_wrap().y;
            ui.vertical_centered(|ui| {
                ui.add_space(vertical_space / 5.0);
                ui.heading(RichText::new("INTER-DIMENSIONAL").size(100.0));
                ui.heading(RichText::new("CURIOSITIES TRADER").size(100.0));

                ui.add_space(50.0);
                let debug_response = ui.add(
                    Button::new(RichText::new("Debug").size(50.0))
                        .min_size(Vec2::new(400.0, 100.0)),
                );

                if debug_response.clicked() {
                    self.on_debug_clicked(ctx, state);
                }
            });
        });
    }
}
