use crate::state::AppState;
use crate::views::{View, ViewManager};
use eframe::{App, Frame, Storage};
use egui::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IDCTApp {
    state: AppState,
    view_manager: ViewManager,
}

impl IDCTApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();

        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.family = egui::FontFamily::Monospace;
        }

        cc.egui_ctx.set_style(style);

        if let Some(storage) = cc.storage {
            if let Some(app) = eframe::get_value::<Self>(storage, eframe::APP_KEY) {
                return app;
            }
        }

        Self::default()
    }
}

impl App for IDCTApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.view_manager.render(ctx, &mut self.state);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
