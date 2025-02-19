use eframe::egui::{self, Key, RichText};
use ringbuf::StaticRb;
use std::{collections::HashSet, fs::File, io::Read};

use crate::database::Database;
use draw::side_panel::{DictionaryPopUp, TranslatePopUp};

mod draw;
mod input;
mod text_utils;

pub struct MyEguiApp {
    lines: Vec<String>,
    index: usize,
    paragraph: Vec<RichText>,
    location_id: egui::Id,

    database: Database,

    dictionary_pop_up: DictionaryPopUp,
    translate_pop_up: TranslatePopUp,

    translate_history: StaticRb<(String, String), 100>,

    prev_keys_down: HashSet<Key>,
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
            location_id: egui::Id::new("location id"),

            database: Database::new(),

            dictionary_pop_up: DictionaryPopUp::new(),
            translate_pop_up: TranslatePopUp::new(),

            translate_history: StaticRb::<(String, String), 100>::default(),

            prev_keys_down: HashSet::new(),
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
