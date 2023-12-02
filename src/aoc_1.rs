use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn print() {
    let file = File::open("./src/01.txt");
    let reader = BufReader::new(file.unwrap());
    let sum = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| digits(&s))
        .map(|v| v.first().unwrap() + v.last().unwrap())
        .reduce(|a,b| a + b)
        .unwrap();
    println!("{}", sum);
}

fn digits(s: &str) -> Vec<u32> {
    return s.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
}

