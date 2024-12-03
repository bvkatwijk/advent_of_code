advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result: usize = input.lines()
        .map(|l| parse_vec_u32(l))
        .filter(|v| safe(v))
        .count();
    u32::try_from(result).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn parse_vec_u32(input: &str) -> Vec<u32> {
    let numbers: Result<Vec<u32>, _> = input.split_whitespace()
        .map(str::parse)
        .collect();
    numbers.unwrap()
}

pub fn safe(nums: &Vec<u32>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vec_u32() {
        assert_eq!(vec![1, 2], parse_vec_u32("1 2"));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
