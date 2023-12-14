use core::num;

use crate::helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_6/example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_6/01.txt";

#[allow(dead_code)]
fn aoc_6_1(path: &str) -> u64 {
    races(path)
        .iter()
        .map(|r| ways_to_win(r))
        .reduce(|a, b| a * b)
        .unwrap()
}

fn ways_to_win(race: &Race) -> u64 {
    2
}

fn races(path: &str) -> Vec<Race> {
    let number_lines: Vec<Vec<u16>> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .map(|s| line_to_numers(s))
        .collect();
    let race_count = number_lines[0].len();
    (0..race_count)
        .into_iter()
        .map(|i| Race {
            time: number_lines[0][i],
            record: number_lines[1][i],
        })
        .collect()
}

fn line_to_numers(s: String) -> Vec<u16> {
    s.split(":")
        .skip(1)
        .map(|s| s.trim())
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.trim())
        .map(|s| s.parse::<u16>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Race {
    time: u16,
    record: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_6_1_test() {
        assert_eq!(288, aoc_6_1(EXAMPLE_01));
    }

    #[test]
    fn line_to_numers_test() {
        assert_eq!(
            vec![7, 15, 30],
            line_to_numers("Time:      7  15   30".to_string())
        );
    }
}
