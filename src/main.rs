use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use eframe::egui::{self, Color32, RichText};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Tongue",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .unwrap();
}

struct MyEguiApp {
    lines: Lines<BufReader<File>>,
    paragraph: Vec<RichText>,
    word_list: Vec<String>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let file = File::open("book.txt").expect("Failed to read file");
        let reader = BufReader::new(file);
        let lines = reader.lines();
        Self {
            lines,
            paragraph: vec![],
            word_list: vec![],
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|input_state| {
            if input_state.key_pressed(egui::Key::ArrowDown) {
                let text = next_paragraph(&mut self.lines);
                self.paragraph = text
                    .split(" ")
                    .map(|token| {
                        let word = token
                            .chars()
                            .filter(|c| c.is_alphabetic())
                            .collect::<String>()
                            .to_lowercase();

                        if self.word_list.contains(&word) {
                            return RichText::from(token).color(Color32::GREEN);
                        }
                        RichText::from(token)
                    })
                    .collect();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reading Area");

            // ? Why do I need horizontal with labels?
            ui.horizontal_wrapped(|ui| {
                // ! Try to remove clone here
                for token in self.paragraph.clone().into_iter() {
                    ui.label(token);
                }
            })
        });
    }
}

fn next_paragraph(lines: &mut Lines<BufReader<File>>) -> String {
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => continue,
        };

        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("Error while reading a line {e}");
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        return line;
    }
}
