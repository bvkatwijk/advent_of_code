use std::{collections::HashMap, cmp::Ordering, ops::Index};

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
    let split: Vec<&str> = s.split_whitespace()
        .collect();
    HandBid {
        hand: split[0]
            .to_lowercase()
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            }),
        orig: split[0].to_string(),
        bid: split[1].parse::<u16>().unwrap(),
    }
}

struct HandBid {
    hand: HashMap<char, u8>,
    orig: String,
    bid: u16
}

impl HandBid {
    fn compare(&self, other: &HandBid) -> Ordering {
        self.hand_type_order(other).then(self.hand_card_order(other))
    }

    // Compare hand type (e.g. four of a kind > full house)
    fn hand_type_order(&self, other: &HandBid) -> Ordering {
        self.hand.values().max().cmp(&other.hand.values().max())
    }

    // Compare hand card (e.g. A > K)
    fn hand_card_order(&self, other: &HandBid) -> Ordering {
        hand_card_compare(&self.orig, &other.orig)
    }
}

fn hand_card_compare(one: &str, other: &str) -> Ordering {
    let ord = card_compare(&one[0..1], &other[0..1]);
    match one.len()  {
        1 => ord,
        _ => match ord {
            Ordering::Equal => hand_card_compare(&one[1..], &other[1..]),
            _ => ord
        }
    }
}

fn card_compare(one: &str, other: &str) -> Ordering {
    score(one).cmp(&score(&other))   
}

fn score(str: &str) -> usize {
    "23456789TJQKA"
        .find(str)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_7_1_test() {
        assert_eq!(6440, aoc_7_1(EXAMPLE));
    }

    #[test]
    fn hand_type_order_test() {
        let h_aaaaa = as_hand("AAAAA 0");
        let h_22222 = as_hand("22222 0");
        let h_aaaak = as_hand("AAAAK 0");
        let h_kkkka = as_hand("KKKKA 0");
        let h_aaakq = as_hand("AAAKQ 0");
        let h_aaaqq = as_hand("AAAQQ 0");
        assert_eq!(Ordering::Greater, h_aaaaa.hand_type_order(&h_aaaak));
        assert_eq!(Ordering::Less, h_aaaak.hand_type_order(&h_aaaaa));
        assert_eq!(Ordering::Less, h_aaaak.hand_type_order(&h_22222));
        assert_eq!(Ordering::Equal, h_aaaaa.hand_type_order(&h_aaaaa));
        assert_eq!(Ordering::Equal, h_aaaak.hand_type_order(&h_kkkka));
        assert_eq!(Ordering::Greater, h_aaaqq.hand_type_order(&h_aaakq));
    }

    #[test]
    fn hand_compare_test() {
        let h_aaaaa = as_hand("AAAAA 0");
        let h_22222 = as_hand("22222 0");
        let h_aaaak = as_hand("AAAAK 0");
        let h_kkkka = as_hand("KKKKA 0");
        let h_aaakq = as_hand("AAAKQ 0");
        let h_aaaqq = as_hand("AAAQQ 0");
        assert_eq!(Ordering::Greater, h_aaaaa.compare(&h_22222));
        assert_eq!(Ordering::Greater, h_22222.compare(&h_aaaak));
    }

    #[test]
    fn hand_card_order_test() {
        let one = as_hand("AAAAA 0");
        let two = as_hand("AAAAK 0");
        let three = as_hand("KKKKA 0");
        assert_eq!(Ordering::Greater, one.hand_card_order(&two));
        assert_eq!(Ordering::Less, two.hand_card_order(&one));
        assert_eq!(Ordering::Equal, one.hand_card_order(&one));
        assert_eq!(Ordering::Greater, two.hand_card_order(&three));
    }

    #[test]
    fn card_compare_test() {
        assert_eq!(Ordering::Equal, card_compare("A", "A"));
        assert_eq!(Ordering::Greater, card_compare("A", "K"));
        assert_eq!(Ordering::Greater, card_compare("K", "Q"));
        assert_eq!(Ordering::Greater, card_compare("Q", "J"));
        assert_eq!(Ordering::Greater, card_compare("J", "T"));
        assert_eq!(Ordering::Greater, card_compare("T", "9"));
        assert_eq!(Ordering::Greater, card_compare("9", "8"));
    }
}