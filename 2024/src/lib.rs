use core::fmt;

pub mod template;

#[derive(Debug, PartialEq, Eq)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl fmt::Display for Point {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {}", self.x, self.y)
    }
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

pub fn pairs<T : Clone>(vec: &[T]) -> Vec<(T, T)> {
    let mut result: Vec<(T, T)> = Vec::new();
    for (i, e) in vec.iter().enumerate() {
        if i < vec.len() - 1 {
            vec[i+1..].iter()
                .for_each(|f| result.push((e.clone(), f.clone())));
        }
    }
    result
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

    #[test]
    fn test_pairs() {
        assert_eq!(
            pairs(&vec![1,2,3]),
            vec![(1,2), (1,3), (2,3)]
        );
    }
}
