advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result: usize = input
        .lines()
        .map(parse_vec_u32)
        .filter(|v| safe(v))
        .count();
    u32::try_from(result).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: usize = input
        .lines()
        .map(parse_vec_u32)
        .filter(|v| safe_with_problem_dampener(v))
        .count();
    u32::try_from(result).ok()
}

pub fn parse_vec_u32(input: &str) -> Vec<u32> {
    let numbers: Result<Vec<u32>, _> = input.split_whitespace().map(str::parse).collect();
    numbers.unwrap()
}

// map to all differences, must all be same sign and within 1--3
pub fn safe(nums: &[u32]) -> bool {
    let mut increasing: Option<bool> = None;
    for window in nums.windows(2) {
        let diff_and_inc = analyze_window(window);
        if diff_and_inc.0 {
            return false;
        }

        match increasing {
            Some(b) => {
                if diff_and_inc.1 != b {
                    return false;
                }
            }
            None => increasing = Some(window[0] > window[1]),
        }
    }
    true
}
// when failure detected, remove the problem element
// if window is the first one, attempt with both first or second element removed, otherwise, remove second
pub fn safe_with_problem_dampener(nums: &[u32]) -> bool {
    safe(nums) || safe_bar_one(nums)
}

pub fn diffs(num: &[u32]) -> Vec<i32> {
    num.windows(2).map(|w| w[0] as i32 - w[1] as i32).collect()
}

pub fn safe_bar_one(nums: &[u32]) -> bool {
    for (pos, _el) in nums.iter().enumerate() {
        let mut new_nums = nums.to_owned();
        new_nums.remove(pos);
        // return as soon as it is safe
        if safe(&new_nums) {
            return true;
        }
    }
    false
}
// returns whether difference is ok, and whether it is increasing
pub fn analyze_window(window: &[u32]) -> (bool, bool) {
    let diff = window[0].abs_diff(window[1]);
    (!(1..=3).contains(&diff), window[0] > window[1])
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
    fn test_safe() {
        assert_eq!(safe(&vec![1, 2, 3]), true)
    }

    #[test]
    fn test_safe_bar_one() {
        assert_eq!(safe_bar_one(&vec![1, 5, 2, 3]), true)
    }

    #[test]
    fn test_safe_with_problem_dampener() {
        assert_eq!(safe_with_problem_dampener(&vec![1, 5, 2, 3]), true);
        assert_eq!(safe_with_problem_dampener(&vec![1, 10, 2, 3]), true);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
