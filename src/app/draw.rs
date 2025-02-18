use eframe::egui;

use super::MyEguiApp;

mod central_panel;
mod side_panel;

impl MyEguiApp {
    pub fn draw(&mut self, ctx: &egui::Context) {
        self.draw_side_panel(ctx);

        self.draw_central_panel(ctx);
    }
}
