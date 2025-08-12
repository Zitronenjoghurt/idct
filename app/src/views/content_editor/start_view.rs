use crate::components::window_menu::WindowMenu;
use crate::state::AppState;
use crate::views::View;
use crate::windows::ViewWindow;
use egui::{Context, Id, SidePanel, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StartView {
    data_window: DataWindow,
}

impl StartView {
    fn render_windows(&mut self, ctx: &Context, state: &mut AppState) {
        self.data_window.render(ctx, state);
    }
}

impl View for StartView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.render_windows(ctx, state);
        SidePanel::left("content_editor_start_view_left_panel").show(ctx, |ui| {
            WindowMenu::new("Window Menu")
                .window(&mut self.data_window)
                .show(ui);
        });
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DataWindow {
    open: bool,
}

impl ViewWindow for DataWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_start_view_data_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Data Handling"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        if ui.button("Restore Core Data").clicked() {
            match state.content_editor_state.restore_core_data() {
                Ok(_) => state.toast_success("Core data restored."),
                Err(e) => {
                    state.toast_error(&e.to_string());
                }
            }
        }
    }
}
