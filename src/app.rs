use eframe::egui::{self, RichText};
use ringbuf::StaticRb;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use crate::database::Database;

mod draw;
mod input;
mod text_utils;

pub struct MyEguiApp {
    lines: Lines<BufReader<File>>,
    paragraph: Vec<RichText>,

    database: Database,
    dictionary_open: bool,
    search_text: String,

    text_history: Vec<String>,
    index: usize,
    translate_history: StaticRb<(String, String), 100>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let file = File::open("book.txt").expect("Failed to read file");
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut temp = Self {
            lines,
            paragraph: vec![],

            database: Database::new(),
            dictionary_open: false,
            search_text: String::new(),

            text_history: vec![],
            index: 0,
            translate_history: StaticRb::<(String, String), 100>::default(),
        };
        temp.get_history_entry(0);
        temp
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.input(ctx);

        self.draw(ctx);
    }
}
