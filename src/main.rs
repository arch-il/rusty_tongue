use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use console::{Key, Style, Term};
use rust_translate::translate_to_english;

#[tokio::main]
async fn main() {
    let term = Term::stdout();

    let file = File::open("book.txt").expect("Failed to read file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    draw(&mut lines).await;
    // input(&term);
}

async fn draw(lines: &mut Lines<BufReader<File>>) {
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

        println!("{}\n", line);

        let english_line = translate_to_english(&line).await.unwrap();
        let style = Style::new().black().on_white();
        println!("{}\n", style.apply_to(english_line));

        break;
    }
}

fn input(term: &Term) {
    loop {
        let key = term.read_key();
        match key {
            Ok(key) => {
                if key == Key::ArrowRight {
                    break;
                }
            }
            Err(e) => println!("Error while reading a key {e}"),
        }
    }
}
