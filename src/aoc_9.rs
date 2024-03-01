use crate::helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_9/example_01.txt";
#[allow(dead_code)]
const INPUT: &str = "./src/aoc_9/input.txt";

#[allow(dead_code)]
fn aoc_9_1(path: &str) -> i64 {
    helper::file_lines(path)
        .map(Result::unwrap)
        .map(split_whitespace_parse_i64)
        .map(history)
        .sum()
}

#[allow(dead_code)]
fn aoc_9_2(path: &str) -> i64 {
    helper::file_lines(path)
        .map(Result::unwrap)
        .map(split_whitespace_parse_i64)
        .map(history_beginning)
        .sum()
}

fn split_whitespace_parse_i64(l: String) -> Vec<i64> {
    l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
}

fn history(line: Vec<i64>) -> i64 {
    pyramid(line)
        .iter()
        .map(|v| v.last().unwrap())
        .sum()
}

fn history_beginning(line: Vec<i64>) -> i64 {
    pyramid(line)
        .iter()
        .map(|v| v.last().unwrap())
        .sum()
}

fn pyramid(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut lines = vec![line.clone()];
    let mut new_line = line;
    while !new_line.iter().all(|u| u.eq(&0)) {
        new_line = diffs(&new_line);
        lines.push(new_line.clone());
    }
    lines
}

fn diffs(new_line: &Vec<i64>) -> Vec<i64> {
    new_line.windows(2).map(|w| w[1] - w[0]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_9_1_test() {
        assert_eq!(114, aoc_9_1(EXAMPLE_01));
        assert_eq!(1725987467, aoc_9_1(INPUT));
    }

    #[test]
    fn aoc_9_2_test() {
        assert_eq!(5, aoc_9_2(EXAMPLE_01));
    }
}
