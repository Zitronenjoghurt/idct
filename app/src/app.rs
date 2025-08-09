use eframe::{App, Frame};
use egui::Context;

pub struct IDCTApp;

impl IDCTApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self
    }
}

impl App for IDCTApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {}
}
