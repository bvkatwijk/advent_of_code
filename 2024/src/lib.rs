pub mod template;

pub enum Dir {
    North,
    East,
    South,
    West,
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

// Split
pub fn matrix(input: &str) -> Vec<Vec<&str>> {
    input.lines()
    .map(|l| l.split("").collect())
    .collect()
}

// Return row count and col count
pub fn matrix_size(mat: &[Vec<&str>]) -> (usize, usize) {
    (
        mat.len(),
        mat.iter().next().map(Vec::len).unwrap()
    )
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
}
