use std::{cmp, collections::HashMap};

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
    let game_lines = game_lines(path);
    number_of_sheets(game_lines)
}

fn number_of_sheets(game_lines: Vec<String>) -> u32 {
    let total_games = game_lines.len() as u8;
    let mut game_card_count: HashMap<u8, u32> = HashMap::new();

    for (pos, line) in game_lines.iter().enumerate() {
        let game = (pos + 1) as u8;
        let game_weight = *game_card_count.entry(game).or_insert(1);
        let score: u8 = winning_number_count(&line);
        
        for it in 0..score {
            let game_offset = (1 + it + game) as u8;
            if game_offset <= total_games {
                *game_card_count.entry(game_offset).or_insert(1) += game_weight;
            }
        }
    }
    game_card_count.values()
        .map(|u| *u as u32)
        .sum()
}

fn game_lines(path: &str) -> Vec<String> {
    helper::file_lines(&path).map(|l| l.unwrap()).collect()
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
    const GAME_2: &str = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
    const GAME_3: &str = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
    const GAME_4: &str = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
    const GAME_5: &str = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
    const GAME_6: &str = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn aoc_4_1_test() {
        assert_eq!(13, aoc_4_1(EXAMPLE_01));
        assert_eq!(25004, aoc_4_1(ACTUAL));
    }

    #[test]
    fn aoc_4_2_test() {
        assert_eq!(30, aoc_4_2(EXAMPLE_01));
        assert_eq!(14427616, aoc_4_2(ACTUAL));
    }

    #[test]
    fn number_of_sheets_test() {
        assert_eq!(1, number_of_sheets(vec![GAME_1.to_string()]));
        assert_eq!(1, number_of_sheets(vec![GAME_2.to_string()]));
        assert_eq!(1, number_of_sheets(vec![GAME_3.to_string()]));

        assert_eq!(3, number_of_sheets(own(vec![GAME_1, GAME_2])));
        assert_eq!(7, number_of_sheets(own(vec![GAME_1, GAME_2, GAME_3])));
        assert_eq!(15, number_of_sheets(own(vec![GAME_1, GAME_2, GAME_3, GAME_4])));
        assert_eq!(29, number_of_sheets(own(vec![GAME_1, GAME_2, GAME_3, GAME_4, GAME_5])));
        assert_eq!(30, number_of_sheets(own(vec![GAME_1, GAME_2, GAME_3, GAME_4, GAME_5, GAME_6])));
    }

    fn own(lines: Vec<&str>) -> Vec<String>{
        lines.iter().map(|s| s.to_string()).collect()
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
