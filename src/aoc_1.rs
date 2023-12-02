use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

#[allow(dead_code)]
pub fn aoc_1_1() {
    let file = File::open("./src/01.txt");
    let reader = BufReader::new(file.unwrap());
    let lines = reader.lines();
    let sum = lines
        .map(|l| l.unwrap())
        .map(|s| digits(&s))
        .map(|v| (10 * v.first().unwrap()) + v.last().unwrap())
        .reduce(|a,b| a + b)
        .unwrap();
    println!("{}", sum);
}

pub fn aoc_1_2() {
    let lines = file_lines();
}

pub fn file_lines() -> Lines<BufReader<File>> {
    return File::open("./src/01.txt")
        .map(|file| BufReader::new(file))
        .map(|it| it.lines())
        .unwrap();
}

fn digits(s: &str) -> Vec<u32> {
    return s.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
}

#[derive(PartialEq)]
pub enum NumberWords {
    One,
    Two,
}

use std::str::FromStr;
impl FromStr for NumberWords {
    type Err = ();

    fn from_str(input: &str) -> Result<NumberWords, Self::Err> {
        match input {
            "one"  => Ok(NumberWords::One),
            "two"  => Ok(NumberWords::Two),
            _      => Err(()),
        }
    }
}

pub fn word_to_number(s: &str) -> Option<u8> {
    return None;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_01() {
        assert_eq!(1, digits("1")[0]);
        assert_eq!(1, digits("a1")[0]);
        assert_eq!(1, digits("a1b")[0]);
        assert_eq!(1, digits("1b")[0]);
    }

    #[test]
    fn word_to_number_01() {
        assert_eq!(None, word_to_number("a"));
        assert_eq!(None, word_to_number("1"));
        assert_eq!(Some(1), word_to_number("One"));
    }
}
