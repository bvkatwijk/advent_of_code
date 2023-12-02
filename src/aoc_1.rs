use std::fs::File;
use std::io::{prelude::*, BufReader, Lines, Result, Error};

const EXAMPLE_01: &str = "./src/01_01_example.txt";
const EXAMPLE_02: &str = "./src/01_02_example.txt";
const ACTUAL: &str = "./src/01.txt";

#[allow(dead_code)]
fn aoc_1_1(input: &str) -> u32 {
    return file_lines(input)
        .map(|l| l.unwrap())
        .map(|s| word_value_01(&s))
        .reduce(|a,b| a + b)
        .unwrap();
}

#[allow(dead_code)]
fn aoc_1_2(input: &str) -> u32 {
    return file_lines(input)
        .map(|l| l.unwrap())
        .map(|s| replace_number_words(&s))
        .map(|s| word_value_01(&s))
        .reduce(|a,b| a + b)
        .unwrap();
}

fn word_value_02(input: &str) -> u32 {
    return word_value_01(&replace_number_words(input));
}

fn word_value_01(input: &str) -> u32 {
    let v = digits(input);
    return (10 * v.first().unwrap()) + v.last().unwrap();
}


// Replace number words with digit, from left to right
fn replace_number_words(input: &str) -> String {
    let mut str = String::from("");
    for c in input.chars() {
        str.push(c);
        str = replace_number_words_smallest_first(&str.as_str());
    }
    return str.to_string();
}

// Replace number words with digit, from small to large
fn replace_number_words_smallest_first(input: &str) -> String {
    let all_digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut str = input.to_string();
    for i in all_digits.iter() {
        str = replace_number(&str, *i);
    }
    return str.to_string();
}

fn file_lines(path: &str) -> Lines<BufReader<File>> {
    return File::open(path)
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

fn replace_number(input: &str, target: u8) -> String {
    let target_str = number_to_word(&target).unwrap();
    return input.replace(target_str, &word_to_number(target_str).unwrap().to_string());
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1_1_result() {
        assert_eq!(142, aoc_1_1(EXAMPLE_01));
        assert_eq!(55712, aoc_1_1(ACTUAL));
    }

    #[test]
    fn aoc_1_2_result() {
        assert_eq!(281, aoc_1_2(EXAMPLE_02));
        assert_eq!(1, aoc_1_2(ACTUAL)); // WIP
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

    #[test]
    fn replace_number_words_test() {
        assert_eq!("1", replace_number_words("one"));
        assert_eq!("12", replace_number_words("onetwo"));
        assert_eq!("219", replace_number_words("two1nine"));
    }

    #[test]
    fn word_value_01_test() {
        assert_eq!(12, word_value_01("1abc2"));
        assert_eq!(38, word_value_01("pqr3stu8vwx"));
        assert_eq!(15, word_value_01("a1b2c3d4e5f"));
        assert_eq!(77, word_value_01("treb7uchet"));
    }

    #[test]
    fn word_value_02_test() {
        assert_eq!(29, word_value_02("two1nine"));
        assert_eq!(83, word_value_02("eightwothree"));
        assert_eq!(13, word_value_02("abcone2threexyz"));
        assert_eq!(24, word_value_02("xtwone3four"));
        assert_eq!(42, word_value_02("4nineeightseven2"));
        assert_eq!(14, word_value_02("zoneight234"));
        assert_eq!(76, word_value_02("7pqrstsixteen"));
    }
}
