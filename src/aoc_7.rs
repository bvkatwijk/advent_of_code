use std::{collections::HashMap, cmp::Ordering};

use crate::helper;


#[allow(dead_code)]
const EXAMPLE: &str = "./src/aoc_7/example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_7/01.txt";

#[allow(dead_code)]
fn aoc_7_1(path: &str) -> usize {
    let mut hands: Vec<HandBid> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .map(|l| as_hand(&l))
        .collect();
    hands.sort_by(|a, b| a.compare(b));
    hands.reverse();
    hands.iter()
        .enumerate()
        .map(|(i, h)| i * h.bid as usize)
        .sum()
}

fn as_hand(s: &str) -> HandBid {
    let split: Vec<&str> = s.split_whitespace().collect();
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
}

struct HandBid {
    hand: HashMap<char, u8>,
    bid: u16
}

impl HandBid {
    fn compare(&self, other: &HandBid) -> Ordering {
        self.hand_type_order(other)
    }

    fn hand_type_order(&self, other: &HandBid) -> Ordering {
        self.hand.values().max().cmp(&other.hand.values().max())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_7_1_test() {
        // assert_eq!(6440, aoc_7_1(EXAMPLE));
    }

    #[test]
    fn hand_type_order_test() {
        let one = as_hand("AAAAA 0");
        let other = as_hand("AAAAK 0");
        assert_eq!(Ordering::Greater, one.hand_type_order(&other));
        assert_eq!(Ordering::Less, other.hand_type_order(&one));
        assert_eq!(Ordering::Equal, one.hand_type_order(&one));
    }
}