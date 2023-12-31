use core::panic;
use std::{cmp::Ordering, collections::HashMap};

use crate::helper::{self};

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_8/example_01.txt";
#[allow(dead_code)]
const EXAMPLE_02: &str = "./src/aoc_8/example_02.txt";

#[allow(dead_code)]
fn aoc_8_1(path: &str) -> usize {
    let mut lines: Vec<String> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .collect();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_8_1_test() {
        assert_eq!(2, aoc_8_1(EXAMPLE_01));
    }
}
