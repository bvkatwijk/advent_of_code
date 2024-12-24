pub mod template;

#[derive(Debug, PartialEq, Eq)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

// Split
pub fn matrix(input: &str) -> Vec<Vec<&str>> {
    input.lines()
    .map(|l| l.split("").filter(|s| !s.is_empty()).collect())
    .collect()
}

// Return row count and col count
pub fn matrix_size(mat: &[Vec<&str>]) -> (usize, usize) {
    (
        mat.len(),
        mat.iter().next().map(Vec::len).unwrap()
    )
}

// Count number of occurrences of a str in a matrix
pub fn count(mat: &[Vec<&str>], arg: &str) -> u32 {
    mat.iter()
        .flat_map(move |r| r.iter())
        .filter(|s| **s == arg)
        .count()
        .try_into()
        .unwrap()
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
    fn test_count() {
        assert_eq!(count(&vec![
            vec![".", "a"],
            vec!["b", "."]
        ], "b"),
        1);
    }
}
