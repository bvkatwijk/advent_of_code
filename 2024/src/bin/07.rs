advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines()
        .filter_map(|l| calibration(l, false))
        .sum())
}

pub fn calibration(input: &str, with_concat: bool) -> Option<u64> {
    let mut s = input.split(":");
    let sum  = str::parse::<u64>(s.next().unwrap()).ok().unwrap();
    let vals: Vec<u64> = s.next()
        ?.split_whitespace()
        .map(|s| str::parse::<u64>(s).ok().unwrap())
        .collect();

    if !with_concat && combination(vals[0], sum, &vals[1..]) {
        return Some(sum);
    } else if with_concat && combination_with_concat(vals[0], sum, &vals[1..]) {
        return Some(sum);
    } else {
        return None;
    }
}

pub fn combination(current: u64, target: u64, vals: &[u64]) -> bool {
    if current == target && vals.len() < 1 {
        return true;
    }
    if current > target {
        return false;
    }
    if vals.len() < 1 {
        return false;
    }
    let next: u64 = vals[0];
    let remaining = &vals[1..];
    return combination(current + next, target, &remaining)
        || combination(current * next, target, &remaining);
}

pub fn combination_with_concat(current: u64, target: u64, vals: &[u64]) -> bool {
    if current == target && vals.len() < 1 {
        return true;
    }
    if current > target {
        return false;
    }
    if vals.len() < 1 {
        return false;
    }
    let next: u64 = vals[0];
    let remaining = &vals[1..];
    return combination_with_concat(current + next, target, &remaining)
        || combination_with_concat(current * next, target, &remaining)
        || combination_with_concat(concat(current, next), target, &remaining);
}

pub fn concat(one: u64, other: u64) -> u64 {
    str::parse::<u64>(&(one.to_string() + &other.to_string()))
        .ok()
        .unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines()
        .filter_map(|l| calibration(l, true))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration() {
        assert_eq!(calibration("190: 10 19", false), Some(190));
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
