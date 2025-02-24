use eframe::egui::{self, Vec2};

use crate::app::MyEguiApp;

impl MyEguiApp {
    pub fn draw_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Progress").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let pos = self.page_location + 1;
                let max = self.lines.len();

                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(3.5, 3.0);

                    ui.label("loc:");

                    let mut input_loc = self.page_location.to_string();
                    egui::TextEdit::singleline(&mut input_loc)
                        .clip_text(false)
                        .desired_width(0.0)
                        .id(self.location_box_id)
                        .show(ui);
                    let loc_text = input_loc.parse().unwrap_or(self.page_location);
                    if loc_text != self.page_location {
                        self.page_location = loc_text;
                        self.refresh_page();
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
