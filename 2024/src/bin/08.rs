use std::{collections::HashMap, iter::Map};

use advent_of_code::{matrix, Point};


advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mat = matrix(input);
    let antennas = antennas(&mat);
    None
}

fn antennas(mat: &[Vec<&str>]) -> HashMap<char, Vec<Point>> {
    let map: HashMap<char, Vec<Point>> = HashMap::new();
    map
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_antennas() {
        let point = Point { x: 1, y: 2 };
        let mut map = HashMap::new();
        map.entry("a").insert_entry(&point);
        assert_eq!(antennas(&vec![
            vec![".", "a"],
            vec!["b", "."]
        ]).get(&'a').unwrap(),
        &vec![point]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
