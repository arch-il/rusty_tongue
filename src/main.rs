mod app;
use app::MyEguiApp;

mod database;

use eframe::egui::{Vec2, ViewportBuilder};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = ViewportBuilder::default().with_inner_size(Vec2::new(600.0, 400.0));

    eframe::run_native(
        "Rusty Tongue",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .unwrap();
}
