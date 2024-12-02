use std::{collections::HashMap, hash::Hash, ops::Index};

advent_of_code::solution!(1);

pub struct Entry {
    left: u32,
    right: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Entry> = input.lines()
        .flat_map(|l| parse(l))
        .collect();

    let mut first_list: Vec<u32> = Vec::with_capacity(lines.len());
    let mut sec_list: Vec<u32> = Vec::with_capacity(lines.len());
    for ele in lines {
        first_list.push(ele.left);
        sec_list.push(ele.right);
    }

    first_list.sort();
    sec_list.sort();

    let mut diff: u32 = 0;
    for (pos, e) in first_list.iter().enumerate() {
        diff += first_list[pos].abs_diff(sec_list[pos])
    }

    Some(diff)
}

pub fn parse(input: &str) -> Result<Entry, &'static str> {
    let mut parts = input.split_whitespace();
    let left = parts.next().ok_or("Bad input")?.parse::<u32>().map_err(|_| "invalid: first")?;
    let right = parts.next().ok_or("Bad input")?.parse::<u32>().map_err(|_| "invalid: second")?;
    Ok(Entry { left, right})
}


pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Entry> = input.lines()
        .flat_map(|l| parse(l))
        .collect();

    let mut first_list: Vec<u32> = Vec::with_capacity(lines.len());
    let mut sec_hist: HashMap<u32, u32> = HashMap::new();
    for ele in lines {
        first_list.push(ele.left);
        *sec_hist.entry(ele.right).or_default() += 1;
    }

    let mut diff: u32 = 0;
    for (pos, e) in first_list.iter().enumerate() {
        diff += first_list[pos] * sec_hist.get(&first_list[pos]).unwrap_or(&0);
    }
    Some(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1 1"), Some(0));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
