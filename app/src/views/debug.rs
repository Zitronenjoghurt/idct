use crate::state::AppState;
use crate::views::View;
use egui::{CentralPanel, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DebugView;

impl View for DebugView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Debug View");
        });
    }
}
