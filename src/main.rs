use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("book.txt").expect("Failed to read file");
    let reader = BufReader::new(file);

    for line in reader.lines().into_iter().take(200) {
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

        println!("{}", line);
    }
}
