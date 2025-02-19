use eframe::egui::{self, Vec2};

use crate::app::MyEguiApp;

impl MyEguiApp {
    pub fn draw_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Progress").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let pos = self.location + 1;
                let max = self.lines.len();

                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(3.5, 3.0);

                    ui.label("loc:");

                    let mut search_text = self.location.to_string();
                    egui::TextEdit::singleline(&mut search_text)
                        .clip_text(false)
                        .desired_width(0.0)
                        .id(self.location_box_id)
                        .show(ui);
                    let temp = search_text.parse().unwrap_or(self.location);
                    if temp != self.location {
                        self.location = temp;
                        self.get_history_entry();
                    }

                    ui.label("/");
                    ui.label((max - 1).to_string());
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.1}%", (pos * 100) as f32 / max as f32));
                });
            })
        });
    }
}
