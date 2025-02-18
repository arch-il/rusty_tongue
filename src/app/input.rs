use eframe::egui;

use super::MyEguiApp;

impl MyEguiApp {
    pub fn input(&mut self, ctx: &egui::Context) {
        ctx.input(|input_state| {
            if input_state.key_pressed(egui::Key::ArrowDown) {
                self.index += 1;
                self.get_history_entry();
            }

            if input_state.key_pressed(egui::Key::ArrowUp) && self.index != 0 {
                self.index -= 1;
                self.get_history_entry();
            }
        });
    }
}
