use std::collections::HashMap;

use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_2/02_01_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_2/02_01.txt";

const RED_MAX: u8 = 12;
const BLUE_MAX: u8 = 14;
const GREEN_MAX: u8 = 13;

#[allow(dead_code)]
fn aoc_2_1(input: &str) -> u32 {
    helper::file_lines(input)
        .map(|l| l.unwrap())
        .map(|s| possible_game_id_or_0(&s))
        .map(|i| (i) as u32)
        .sum()
}

fn possible_game_id_or_0(input: &str) -> u8 {
    let all_possible = draws(input)
        .iter()
        .map(|s| draw(s))
        .all(|d| is_possible(&d));
    if all_possible {
        game_id(input)
    } else {
        0
    }
}

fn is_possible(draw: &HashMap<Color, u8>) -> bool {
    draw.get(&Color::Blue).unwrap_or(&0) <= &BLUE_MAX
        && draw.get(&Color::Red).unwrap_or(&0) <= &RED_MAX
        && draw.get(&Color::Green).unwrap_or(&0) <= &GREEN_MAX
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

// fn color_to_str(color: Color) -> &'static str {
//     match color {
//         Color::Red => "red",
//         Color::Green => "green",
//         Color::Blue => "blue"
//     }
// }

fn str_to_color(input: &str) -> Option<Color> {
    match input {
         "red" => Some(Color::Red),
         "green" => Some(Color::Green),
         "blue" => Some(Color::Blue),
         _ => None,
    }
}

fn draws(input: &str) -> Vec<&str> {
    let split: Vec<&str> = input.split(":").collect();
    split[1]
        .split(";")
        .map(|s| s.trim())
        .collect()
}

// Parse single draw into map of color to count
fn draw(input: &str) -> HashMap<Color, u8> {
    let mut draw = HashMap::new();
    input.split(",")
        .map(|s| s.trim())
        .map(|s| parse(&s))
        .for_each(|e| {
            let count = draw.entry(e.1).or_insert(0);
            *count += e.0;
        });
    draw
}

// Parse a draw string
// e.g. "3 blue" -> (3, Blue)
fn parse(draw: &str) -> (u8, Color) {
    let split: Vec<&str> = draw.split(" ").collect();
    let count = &split[0].parse::<u8>().unwrap();
    let color = str_to_color(&split[1]).unwrap();
    (*count, color)
}

fn game_id(input: &str) -> u8 {
    input.split(":")
        .next()
        .map(|s| &s[5..])
        .map(|s| s.parse::<u8>().unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::aoc_2::{aoc_2_1, EXAMPLE_01};

    const GAME_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    #[test]
    fn aoc_1_2_example_test() {
        assert_eq!(8, aoc_2_1(EXAMPLE_01));
        assert_eq!(1, aoc_2_1(ACTUAL));
    }

    #[test]
    fn game_id_test() {
        assert_eq!(1, game_id(GAME_1));
        assert_eq!(1, game_id(GAME_1));
    }

    #[test]
    fn draws_test() {
        assert_eq!("3 blue, 4 red", draws(GAME_1)[0]);
        assert_eq!("1 red, 2 green, 6 blue", draws(GAME_1)[1]);
        assert_eq!("2 green", draws(GAME_1)[2]);
    }

    #[test]
    fn str_to_color_test() {
        assert_eq!(Color::Blue, str_to_color("blue").unwrap());
    }

    // #[test]
    // fn color_to_str_test() {
    //     assert_eq!("blue", color_to_str(Color::Blue));
    // }

    #[test]
    fn is_possible_test() {
        let mut draw = HashMap::new();
        draw.insert(Color::Red, 1);
        assert!(is_possible(&draw));
    }

    #[test]
    fn parse_test() {
        assert_eq!((1, Color::Blue), parse("1 blue"));
        assert_eq!((1, Color::Red), parse("1 red"));
        assert_eq!((3, Color::Blue), parse("3 blue"));
    }

    #[test]
    fn draw_test() {
        let mut expect = HashMap::new();
        expect.insert(Color::Red, 1);
        expect.insert(Color::Blue, 6);
        assert_eq!(expect, draw("1 red, 6 blue"));
    }
}