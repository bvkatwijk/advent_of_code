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

fn aoc_6_2(path: &str) -> u64 {
    let result: Vec<u64> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .map(|s| line_to_numbers(s))
        .map(|v| helper::concat_numbers(v))
        .collect();
    ways_to_win(&Race{
        time: result[0],
        record: result[1]
    })
}

fn ways_to_win(race: &Race) -> u64 {
    (0..race.time).into_iter()
        .filter(|i| i * (race.time - i) > race.record)
        .count() as u64
}

fn races(path: &str) -> Vec<Race> {
    let number_lines: Vec<Vec<u64>> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .map(|s| line_to_numbers(s))
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

fn line_to_numbers(s: String) -> Vec<u64> {
    s.split(":")
        .skip(1)
        .map(|s| s.trim())
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.trim())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Race {
    time: u64,
    record: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_6_1_test() {
        assert_eq!(288, aoc_6_1(EXAMPLE_01));
        assert_eq!(160816, aoc_6_1(ACTUAL));
    }

    #[test]
    fn aoc_6_2_test() {
        assert_eq!(71503, aoc_6_2(EXAMPLE_01));
        assert_eq!(46561107, aoc_6_2(ACTUAL));
    }

    #[test]
    fn line_to_numbers_test() {
        assert_eq!(
            vec![7, 15, 30],
            line_to_numbers("Time:      7  15   30".to_string())
        );
    }

    #[test]
    fn ways_to_win_test() {
        assert_eq!(4, ways_to_win(&Race { time: 7, record: 9 }));
    }
}
