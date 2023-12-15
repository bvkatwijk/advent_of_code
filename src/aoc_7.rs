use std::collections::HashMap;

use crate::helper;


#[allow(dead_code)]
const EXAMPLE: &str = "./src/aoc_7/example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_7/01.txt";

#[allow(dead_code)]
fn aoc_7_1(path: &str) -> u64 {
    let hands: Vec<HandBid> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            HandBid {
                hand: split[0]
                    .to_lowercase()
                    .chars()
                    .fold(HashMap::new(), |mut map, c| {
                        *map.entry(c).or_insert(0) += 1;
                        map
                    }),
                bid: split[1].parse::<u16>().unwrap(),
            }
        })
        .collect();
    // hands.sort_by(|a, b| )
    0
}

struct HandBid {
    hand: HashMap<char, u8>,
    bid: u16
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_7_1_test() {
        assert_eq!(6440, aoc_7_1(EXAMPLE));
    }
}