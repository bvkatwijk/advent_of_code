use std::{collections::HashMap, iter::Map};

use advent_of_code::{count, matrix, Point};


advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mat = matrix(input);
    let antennas = antennas(&mat);

    // iterate over same antennas
    // for each pair
    //attempt to insert antinode

    // coiunt antinodes

    Some(count(&mat, "#"))
}

fn antennas(mat: &[Vec<&str>]) -> HashMap<String, Vec<Point>> {
    let mut map: HashMap<String, Vec<Point>> = HashMap::new();
    mat.iter()
        .enumerate()
        .flat_map(|(r_i, r_e)| r_e.iter()
            .enumerate()
            .map(move |(c_i, c_e)| (r_i, r_e, c_i, *c_e)))
        .for_each(|(r_i, r_e, c_i, c_e)| {
            if c_e != "." {
                println!("current: ({r_i}, {c_i}): {c_e}");
                map.entry(c_e.to_string())
                    .or_insert(Vec::new())
                    .push(Point { x: r_i as u32, y: c_i as u32 });
            }
        });
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
        let p01 = Point { x: 0, y: 1 };
        let p10 = Point { x: 1, y: 0 };
        let result = antennas(&vec![
            vec![".", "a"],
            vec!["b", "."]
        ]);
        assert_eq!(result.get("a").unwrap(), &vec![p01]);
        assert_eq!(result.get("b").unwrap(), &vec![p10]);
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
