#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe;

mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "evisuals",
        native_options,
        Box::new(|cc| Box::new(app::AppState::new(cc))),
    );
}
