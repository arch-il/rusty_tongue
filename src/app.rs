use eframe::egui::{self, Key, RichText};
use ringbuf::{
    StaticRb,
    traits::{Consumer, Producer},
};
use std::{collections::HashSet, fs};

use crate::{database::Database, savestate::Savestate};
use draw::side_panel::DictionaryPopUp;

mod draw;
mod input;
mod text_utils;

pub struct MyEguiApp {
    lines: Vec<String>,
    page_location: usize,
    paragraph: Vec<RichText>,
    word_location: usize,
    location_box_id: egui::Id,

    database: Database,

    dictionary_pop_up: DictionaryPopUp,

    translate_history: StaticRb<String, 100>,

    prev_keys_down: HashSet<Key>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let lines = fs::read_to_string("book.txt")
            .unwrap_or_else(|_| {
                println!("Error: No book present");
                String::new()
            })
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

        let savestate = match fs::read_to_string("savestate.ron") {
            Ok(file) => ron::from_str(&file).unwrap_or_default(),
            Err(_) => Savestate::default(),
        };

        let mut translate_history = StaticRb::<String, 100>::default();
        translate_history.push_iter(savestate.translate_history.into_iter().take(100));

        let mut temp = Self {
            lines,
            page_location: savestate.page_location,
            paragraph: vec![],
            word_location: savestate.word_location,
            location_box_id: egui::Id::new("location id"),

            database: Database::new(),

            dictionary_pop_up: DictionaryPopUp::new(),

            translate_history,
            prev_keys_down: HashSet::new(),
        };
        temp.refresh_page();

        temp
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.input(ctx);

        self.draw(ctx);

        // * for testing frame time
        // println!("{}", ctx.input(|i| i.unstable_dt));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        fs::write(
            "savestate.ron",
            ron::ser::to_string_pretty(
                &Savestate {
                    page_location: self.page_location,
                    word_location: self.word_location,
                    translate_history: self.translate_history.iter().cloned().collect(),
                },
                ron::ser::PrettyConfig::default(),
            )
            .unwrap(),
        )
        .expect("Failed while writing savestate");
    }
}
