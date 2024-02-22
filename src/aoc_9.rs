

use crate::helper::{self, debug};

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_9/example_01.txt";
#[allow(dead_code)]
const INPUT: &str = "./src/aoc_9/input.txt";

#[allow(dead_code)]
fn aoc_9_1(path: &str) -> usize {
    helper::file_lines(path)
        .map(|l| l.unwrap())
        .map(|l| l.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect())
        .map(|l| history(l))
        .sum()
}

fn history(line: Vec<usize>) -> usize {
    let mut lines = vec![line.clone()];
    let mut new_line = line;
    while !new_line.iter().all(|u| u.eq(&0)) {
        new_line = diffs(&new_line);
        lines.push(new_line.clone());
    }
    debug(&lines);
    0
}

fn diffs(new_line: &Vec<usize>) -> Vec<usize> {
    debug(&new_line);
    new_line.windows(2).map(|w| w[1] - w[0]).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_9_1_test() {
        assert_eq!(2, aoc_9_1(EXAMPLE_01));
    }
}