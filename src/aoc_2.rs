use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/02_01_example.txt";

#[allow(dead_code)]
fn aoc_2_1(input: &str) -> u32 {
    return helper::file_lines(input)
        .map(|l| l.unwrap())
        .map(|s| possible_game_id_or_0(&s))
        .sum();
}

fn possible_game_id_or_0(input: &str) -> u32 {
    0
}

// enum Color {
//     Red,
//     Green,
//     Blue
// }

// fn game(input: &str) -> Game {
//     Game {

//     }
// }

struct Draw {
    red_count: u8,
    green_count: u8,
    blue_count: u8
}

fn game_id(input: &str) -> u8 {
    return input.split(":")
        .next()
        .unwrap()[5..]
        .parse::<u8>()
        .unwrap();
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::aoc_2::{aoc_2_1, EXAMPLE_01};

    #[test]
    fn aoc_1_2_example_test() {
        assert_eq!(1, aoc_2_1(EXAMPLE_01));
    }

    #[test]
    fn game_id_test() {
        assert_eq!(1, game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
        assert_eq!(1, game_id("Game 1"));
    }
}