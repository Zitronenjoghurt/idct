use crate::app::IDCTApp;

mod app;
mod components;
mod state;
mod views;
mod windows;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("Inter-Dimensional Curiosities Trader"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "IDCT",
        native_options,
        Box::new(|cc| Ok(Box::new(IDCTApp::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
