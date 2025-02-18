use eframe::egui::{self, RichText};
use ringbuf::StaticRb;
use std::{fs::File, io::Read};

use crate::database::Database;

mod draw;
mod input;
mod text_utils;

pub struct MyEguiApp {
    lines: Vec<String>,
    index: usize,
    paragraph: Vec<RichText>,

    database: Database,
    dictionary_open: bool,
    search_text: String,

    translate_history: StaticRb<(String, String), 100>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut lines = String::new();
        File::open("book.txt")
            .expect("Failed to read file")
            .read_to_string(&mut lines)
            .expect("Failed while reaing a file");
        let lines = lines
            .split("\n")
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            })
            .collect::<Vec<_>>();

        let mut temp = Self {
            lines,
            index: 0,
            paragraph: vec![],

            database: Database::new(),
            dictionary_open: false,
            search_text: String::new(),

            translate_history: StaticRb::<(String, String), 100>::default(),
        };
        temp.get_history_entry();

        temp
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.input(ctx);

        self.draw(ctx);
    }
}
