use crate::state::AppState;
use egui::Context;
use serde::{Deserialize, Serialize};

mod debug;
mod main_menu;

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ViewID {
    #[default]
    MainMenu,
    Debug,
}

pub trait View: Default {
    fn render(&mut self, ctx: &Context, state: &mut AppState);
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ViewManager {
    debug: debug::DebugView,
    main_menu: main_menu::MainMenuView,
}

impl View for ViewManager {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        match state.current_view() {
            ViewID::Debug => self.debug.render(ctx, state),
            ViewID::MainMenu => self.main_menu.render(ctx, state),
        }
    }
}
