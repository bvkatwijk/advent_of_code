use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

#[allow(dead_code)]
pub fn aoc_1_1() -> u32 {
    let file = File::open("./src/01.txt");
    let reader = BufReader::new(file.unwrap());
    let lines = reader.lines();
    return lines
        .map(|l| l.unwrap())
        .map(|s| digits(&s))
        .map(|v| (10 * v.first().unwrap()) + v.last().unwrap())
        .reduce(|a,b| a + b)
        .unwrap();
}

pub fn aoc_1_2() {
    let lines = file_lines();
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
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

fn word_to_number(input: &str) -> Option<u8> {
    match input {
        "zero"  => Some(0),
        "one"  => Some(1),
        "two"  => Some(2),
        "three"  => Some(3),
        "four"  => Some(4),
        "five"  => Some(5),
        "six"  => Some(6),
        "seven"  => Some(7),
        "eight"  => Some(8),
        "nine"  => Some(9),
        _      => None,
    }
}
fn number_to_word(input: &u8) -> Option<&str> {
    match input {
        0 => Some("zero"),
        1 => Some("one"),
        2 => Some("two"),
        3 => Some("three"),
        4 => Some("four"),
        5 => Some("five"),
        6 => Some("six"),
        7 => Some("seven"),
        8 => Some("eight"),
        9 => Some("nine"),
        _      => None,
    }
}

fn replace_number(input: &str, target: u8) -> String {
    let target_str = number_to_word(&target).unwrap();
    return input.replace(target_str, &word_to_number(target_str).unwrap().to_string());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1_result() {
        assert_eq!(55712, aoc_1_1());
    }

    #[test]
    fn digits_test() {
        assert_eq!(1, digits("1")[0]);
        assert_eq!(1, digits("a1")[0]);
        assert_eq!(1, digits("a1b")[0]);
        assert_eq!(1, digits("1b")[0]);
    }

    #[test]
    fn word_to_number_test() {
        assert_eq!(None, word_to_number("a"));
        assert_eq!(None, word_to_number("1"));
        assert_eq!(Some(1), word_to_number("one"));
        assert_eq!(Some(2), word_to_number("two"));
    }

    #[test]
    fn replace_number_test() {
        assert_eq!("a", replace_number("a", 1));
        assert_eq!("a", replace_number("a", 2));

        assert_eq!("on", replace_number("on", 1));
        assert_eq!("on", replace_number("on", 2));

        assert_eq!("0", replace_number("zero", 0));
        assert_eq!("1", replace_number("one", 1));
        assert_eq!("2", replace_number("two", 2));
        assert_eq!("3", replace_number("three", 3));
        assert_eq!("4", replace_number("four", 4));
        assert_eq!("5", replace_number("five", 5));
        assert_eq!("6", replace_number("six", 6));
        assert_eq!("7", replace_number("seven", 7));
        assert_eq!("8", replace_number("eight", 8));
        assert_eq!("9", replace_number("nine", 9));

        assert_eq!("11", replace_number("1one", 1));
        assert_eq!("1one", replace_number("1one", 2));
        assert_eq!("12", replace_number("1two", 2));

        assert_eq!("11", replace_number("one1", 1));
        assert_eq!("1two", replace_number("onetwo", 1));
        assert_eq!("12", replace_number(&replace_number("onetwo", 1), 2));
    }
}
