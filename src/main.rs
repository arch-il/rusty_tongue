use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use console::{Key, Term};

fn main() {
    let term = Term::stdout();

    let file = File::open("book.txt").expect("Failed to read file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
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
}
