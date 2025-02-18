use eframe::egui;

use crate::app::MyEguiApp;

impl MyEguiApp {
    pub fn draw_below_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Progress").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let pos = self.index + 1;
                let max = self.lines.len();

                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(format!("loc: {} / {}", pos, max));
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.1}%", (pos * 100) as f32 / max as f32));
                });
            })
        });
    }
}
