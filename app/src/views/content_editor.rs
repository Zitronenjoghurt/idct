use crate::state::AppState;
use crate::views::{View, ViewID};
use egui::{Button, Context, MenuBar, RichText, TopBottomPanel};
use serde::{Deserialize, Serialize};

mod start_view;
mod tag_editor;

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
enum ContentEditorTab {
    #[default]
    Start,
    TagEditor,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContentEditorView {
    active_tab: ContentEditorTab,
    start_view: start_view::StartView,
    tag_editor: tag_editor::TagEditorView,
}

impl ContentEditorView {
    fn on_home_clicked(&self, state: &mut AppState) {
        state.switch_view(ViewID::MainMenu);
    }
}

impl View for ContentEditorView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        TopBottomPanel::top("content_editor_top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                let home_response = ui.add(Button::new(RichText::new(" 🏠 ").size(20.0)));
                if home_response.clicked() {
                    self.on_home_clicked(state);
                }

                ui.label("Content Editor");

                ui.separator();

                ui.selectable_value(&mut self.active_tab, ContentEditorTab::Start, "📄 Start");

                ui.selectable_value(
                    &mut self.active_tab,
                    ContentEditorTab::TagEditor,
                    "🏷 Tag Editor",
                );
            });
        });

        match self.active_tab {
            ContentEditorTab::Start => self.start_view.render(ctx, state),
            ContentEditorTab::TagEditor => self.tag_editor.render(ctx, state),
        }
    }
}
