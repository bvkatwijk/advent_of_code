use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

// Read file and return lines
pub fn file_lines(path: &str) -> Lines<BufReader<File>> {
    File::open(path)
        .map(|file| BufReader::new(file))
        .map(|it| it.lines())
        .unwrap()
}