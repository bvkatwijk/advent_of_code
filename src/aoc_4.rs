use std::{convert::TryInto, cmp, collections::HashMap};

use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_4/04_01_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_4/04_01.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> i32 {
    helper::file_lines(&path)
        .map(|l| l.unwrap())
        .map(|l| winning_numbers(&l))
        .map(|v| score(&v))
        .sum()
}

#[allow(dead_code)]
fn aoc_4_2(path: &str) -> u32 {
    let game_lines: Vec<String> = helper::file_lines(&path).map(|l| l.unwrap()).collect();
    let total_games = game_lines.len() as u8;
    let mut game_card_count: HashMap<u8, u8> = HashMap::new();

    for (pos, line) in game_lines.iter().enumerate() {
        let game = (pos) as u8;
        println!("Game {}", game);
        let score = winning_number_count(&line);
        
        for it in 0..score {
            let game_offset = (it + game) as u8;
            if game_offset < total_games {
                let game_count = game_card_count.entry(game_offset).or_insert(1);
                *game_count += score;
                println!("Game {} scores {:?}", game_offset, score);
            }
        }
    }
    game_card_count.values().map(|u| *u as u32).sum()
}

// Returns amount of winning numbers in this game string
fn winning_number_count(input: &str) -> u8 {
    winning_numbers(&input).len() as u8
}

// 04-01: Returns score of the winning numbers as offset power of 2
fn score(num: &Vec<u8>) -> i32 {
    let base: i32 = 2;
    let count: u32 = num.len() as u32;
    if count == 0 {
        0
    } else {
        base.pow(cmp::max(0, count - 1))
    }
}

// Returns which winning numbers are in this game string
fn winning_numbers(input: &str) -> Vec<u8> {
    let split = game_and_data(input);
    let split2 = winning_and_expected(split[1]);
    let winning: Vec<u8> = numbers(split2[0]);
    numbers(split2[1])
        .iter()
        .filter(|i| winning.contains(i))
        .map(|i| i.to_owned())
        .collect()
}

// Split input string into game id and data
fn game_and_data(input: &str) -> Vec<&str> {
    input.split(":")
        .map(|s| s.trim())
        .collect()
}

// Split input string into winning numbers and own numbers
fn winning_and_expected(data: &str) -> Vec<&str> {
    data.split("|")
        .map(|s| s.trim())
        .collect()
}

// Split input string into individual numbers
fn numbers(input: &str) -> Vec<u8> {
    input.split_whitespace()
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    const GAME_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

    #[test]
    fn aoc_4_1_test() {
        assert_eq!(13, aoc_4_1(EXAMPLE_01));
        assert_eq!(25004, aoc_4_1(ACTUAL));
    }

    #[test]
    fn aoc_4_2_test() {
        assert_eq!(30, aoc_4_2(EXAMPLE_01));
        // assert_eq!(25004, aoc_4_1(ACTUAL));
    }

    #[test]
    fn numbers_test() {
        assert_eq!(vec![48, 83, 17, 86], numbers("48 83 17 86"));
    }

    #[test]
    fn winning_numbers_test() {
        let expect: Vec<u8> = vec![83, 86, 17, 48];
        assert_eq!(expect, winning_numbers(GAME_1));
    }

    #[test]
    fn winning_and_expected_test() {
        let result = winning_and_expected("41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!("41 48 83 86 17", result[0]);
        assert_eq!("83 86  6 31 17  9 48 53", result[1]);
    }
}
