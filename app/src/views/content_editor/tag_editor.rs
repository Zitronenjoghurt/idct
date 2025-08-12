use crate::components::window_menu::WindowMenu;
use crate::state::AppState;
use crate::views::View;
use crate::windows::ViewWindow;
use egui::{Context, Id, SidePanel, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagEditorView {
    overview_window: OverviewWindow,
}

impl TagEditorView {
    fn render_windows(&mut self, ctx: &Context, state: &mut AppState) {
        self.overview_window.render(ctx, state);
    }
}

impl View for TagEditorView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.render_windows(ctx, state);
        SidePanel::left("content_editor_tag_editor_left_panel").show(ctx, |ui| {
            WindowMenu::new("Window Menu")
                .window(&mut self.overview_window)
                .show(ui);
        });
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct OverviewWindow {
    open: bool,
}

impl ViewWindow for OverviewWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_tag_editor_overview_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Tag Overview"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, _state: &mut AppState) {
        ui.label("Test");
    }
}
