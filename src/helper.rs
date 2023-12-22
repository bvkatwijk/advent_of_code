use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

// Read file and return lines
pub fn file_lines(path: &str) -> Lines<BufReader<File>> {
    File::open(path)
        .map(|file| BufReader::new(file))
        .map(|it| it.lines())
        .unwrap()
}

// Print str and return it, useful in function pipelines
pub fn debug<T: std::fmt::Debug>(s: T) -> T {
    println!("debug: {:#?}", s);
    s
}

pub fn concat_numbers(v: Vec<u64>) -> u64 {
    v.iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse::<u64>()
        .unwrap()
}