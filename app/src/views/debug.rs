use crate::state::AppState;
use crate::views::{View, ViewID};
use egui::{
    Button, CentralPanel, Context, MenuBar, RichText, ScrollArea, TextEdit, TopBottomPanel,
};
use idct_game::curiosity::Curiosity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DebugView {
    console_output: String,
    #[serde(skip)]
    scroll_to_bottom: bool,
}

impl DebugView {
    fn push_to_console(&mut self, message: &str) {
        self.console_output.push_str(message);
        self.console_output.push('\n');
        self.scroll_to_bottom();
    }

    fn clear_console(&mut self) {
        self.console_output.clear();
    }

    fn scroll_to_bottom(&mut self) {
        self.scroll_to_bottom = true;
    }

    fn on_home_clicked(&self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::MainMenu);
    }

    fn on_clear_console_clicked(&mut self) {
        self.clear_console();
    }

    fn on_generate_curiosity_clicked(&mut self) {
        let curiosity = Curiosity::default();
        self.push_to_console(
            &serde_json::to_string_pretty(&curiosity)
                .unwrap()
                .to_string(),
        );
    }
}

impl View for DebugView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        TopBottomPanel::top("debug_top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                let home_response = ui.add(Button::new(RichText::new(" 🏠 ").size(20.0)));
                if home_response.clicked() {
                    self.on_home_clicked(ctx, state);
                }

                ui.label("Debug Mode");
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear Console").clicked() {
                    self.on_clear_console_clicked();
                }

                if ui.button("Generate Curiosity").clicked() {
                    self.on_generate_curiosity_clicked();
                }
            });

            ui.separator();

            let mut scroll_area = ScrollArea::vertical().max_height(ui.available_height());
            if self.scroll_to_bottom {
                scroll_area = scroll_area.stick_to_bottom(true);
                self.scroll_to_bottom = false;
            }

            scroll_area.show(ui, |ui| {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(0, 0, 0))
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline(&mut self.console_output.as_str())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .interactive(false),
                        );
                    });
            });
        });
    }
}
