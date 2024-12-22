advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines()
        .filter_map(|l| calibration(l))
        .sum())
}

pub fn calibration(input: &str) -> Option<u64> {
    let mut s = input.split(":");
    let sum  = str::parse::<u64>(s.next().unwrap()).ok().unwrap();
    let vals: Vec<u64> = s.next()
        ?.split_whitespace()
        .map(|s| str::parse::<u64>(s).ok().unwrap())
        .collect();

    if combination(vals[0], sum, &vals[1..]) {
        return Some(sum);
    } else {
        return None;
    }
}

pub fn combination(current: u64, target: u64, vals: &[u64]) -> bool {
    if current == target {
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

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration() {
        assert_eq!(calibration("190: 10 19"), Some(190));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
