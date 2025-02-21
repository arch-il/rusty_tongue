mod app;
use app::MyEguiApp;

mod database;
mod savestate;

use eframe::egui::{Vec2, ViewportBuilder};

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(Vec2::new(800.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty Tongue",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .unwrap();
}
