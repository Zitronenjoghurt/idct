use crate::app::IDCTApp;

mod app;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "IDCT",
        native_options,
        Box::new(|cc| Ok(Box::new(IDCTApp::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
