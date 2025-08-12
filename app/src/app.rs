use crate::state::AppState;
use crate::views::{View, ViewManager};
use anyhow::Context as AnyhowContext;
use eframe::{App, Frame, Storage};
use egui::Context;
use idct_game::Game;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct IDCTApp {
    #[serde(skip)]
    game: Game,
    state: AppState,
    view_manager: ViewManager,
}

impl IDCTApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self::setup_style(&cc.egui_ctx);

        let mut app = Self::load_or_default(cc.storage);
        app.game.initialize().unwrap();
        app.load_game_state(cc.storage).unwrap();
        app
    }

    fn setup_style(ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.family = egui::FontFamily::Monospace;
        }
        ctx.set_style(style);
        ctx.set_pixels_per_point(1.5);
    }

    fn load_or_default(storage: Option<&dyn Storage>) -> Self {
        storage
            .and_then(|s| eframe::get_value::<Self>(s, eframe::APP_KEY))
            .unwrap_or_default()
    }

    fn load_game_state(&mut self, storage: Option<&dyn Storage>) -> anyhow::Result<()> {
        if let Some(game_state_string) = storage.and_then(|s| s.get_string("game_state")) {
            let game_state =
                ron::from_str(&game_state_string).context("Failed to parse game state")?;
            self.game.state = game_state;
        };
        Ok(())
    }
}

impl App for IDCTApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.state.render(ctx);
        self.view_manager.render(ctx, &mut self.state);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        eframe::set_value(storage, "game_state", &self.game.state);
    }
}
