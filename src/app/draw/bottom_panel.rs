use eframe::egui::{self, Vec2};

use crate::app::MyEguiApp;

impl MyEguiApp {
    pub fn draw_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Progress").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let pos = self.index + 1;
                let max = self.lines.len();

                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(3.0, 0.0);

                    ui.label("loc:");

                    let mut search_text = (self.index + 1).to_string();
                    let text_edit = egui::TextEdit::singleline(&mut search_text)
                        .clip_text(false)
                        .desired_width(0.0);
                    ui.add(text_edit).clicked();
                    let temp = search_text.parse().unwrap_or(self.index) - 1;
                    if temp != self.index {
                        self.index = temp;
                        self.get_history_entry();
                    }

                    ui.label(format!("/ {max}"));
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.1}%", (pos * 100) as f32 / max as f32));
                });
            })
        });
    }
}
