use std::collections::HashMap;

use crate::helper;

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

#[allow(dead_code)]
fn aoc_2_2(input: &str) -> u32 {
    helper::file_lines(input)
        .map(|l| l.unwrap())
        .map(|s| minimal_set(&s))
        .map(|s| power(s))
        .map(|i| (i) as u32)
        .sum()
}

fn minimal_set(input: &str) -> HashMap<Color, u8> {
    let game = game(input);
    let min_red = min_cubes_needed(&game, Color::Red);
    let min_green = min_cubes_needed(&game, Color::Green);
    let min_blue = min_cubes_needed(&game, Color::Blue);
    let mut min = HashMap::new();
    min.insert(Color::Red, *min_red);
    min.insert(Color::Green, *min_green);
    min.insert(Color::Blue, *min_blue);
    min
}

fn power(map: HashMap<Color, u8>) -> u32 {
    map.values()
        .map(|i| (*i) as u32)
        .reduce(|a, b| (a * b))
        .unwrap()
}

// Get mininmal cubes needed of a color for supplied game to be possible
fn min_cubes_needed(game: &Vec<HashMap<Color, u8>>, color: Color) -> &u8 {
    game.iter()
        .map(|m| m.get(&color).unwrap_or(&0))
        .max()
        .unwrap_or(&0)
}

// Get game id if the game is possible, else 0
fn possible_game_id_or_0(input: &str) -> u8 {
    let all_possible = game(input)
        .iter()
        .all(|d| is_possible(&d));
    if all_possible {
        game_id(input)
    } else {
        0
    }
}

fn game(input: &str) -> Vec<HashMap<Color, u8>> {
    draws(input)
        .iter()
        .map(|s| draw(s))
        .collect()
}

// Returns whether game is possible according to max cube count
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

    const GAME_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    #[test]
    fn aoc_1_2_test() {
        assert_eq!(8, aoc_2_1(EXAMPLE_01));
        assert_eq!(2061, aoc_2_1(ACTUAL));
    }

    #[test]
    fn aoc_2_2_test() {
        assert_eq!(2286, aoc_2_2(EXAMPLE_01));
        assert_eq!(72596, aoc_2_2(ACTUAL));
    }

    #[test]
    fn minimal_set_test() {
        let mut expect = HashMap::new();
        expect.insert(Color::Red, 4);
        expect.insert(Color::Green, 2);
        expect.insert(Color::Blue, 6);
        assert_eq!(expect, minimal_set(GAME_1));
    }

    #[test]
    fn power_test() {
        let mut expect = HashMap::new();
        expect.insert(Color::Red, 4);
        expect.insert(Color::Green, 2);
        expect.insert(Color::Blue, 6);
        assert_eq!(48, power(expect));
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