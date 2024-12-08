advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mat = matrix(input);
    let (height, width) = matrix_size(&mat);

    let (guard_x, guard_y) = guard(mat);
    Some(0)
}

fn guard(mat: &[Vec<&str>]) -> (u32, u32) {
    // mat.iter()
    //     .filter(predicate)
    (0, 0)
}

pub fn matrix(input: &str) -> Vec<Vec<&str>> {
    input.lines()
    .map(|l| l.split("").collect())
    .collect()
}

pub fn matrix_size(mat: &[Vec<&str>]) -> (usize, usize) {
    (
        mat.len(),
        mat.iter().next().map(Vec::len).unwrap()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_matrix_size() {
        assert_eq!(matrix_size(&vec![
            vec![".", "."],
            vec!["^", "."]]),
            (2,2)
        )
    }

    #[test]
    fn test_guard() {
        assert_eq!(guard(&vec![
            vec![".", "."],
            vec!["^", "."]]),
            (1,0)
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
