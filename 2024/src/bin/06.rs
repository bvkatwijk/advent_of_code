advent_of_code::solution!(6);

const obstacle: &str = "#";

pub enum Mode {
    North,
    East,
    South,
    West,
}

pub fn next_mode(mode: &Mode) -> Mode {
    match mode {
        Mode::North => Mode::East,
        Mode::East => Mode::South,
        Mode::South => Mode::West,
        Mode::West => Mode::North,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut mat = matrix(input);
    let (height, width) = matrix_size(&mat);
    let mut mode = Mode::North;
    let (mut guard_x, mut guard_y) = guard(&mat);

    while guard_x != 0 && guard_x != height - 1 && guard_y != 0 && guard_y != width - 1 {
        let next = next_move(guard_x, guard_y, &mode);
        let next_value = mat[next.0][next.1];
        if mat[next.0][next.1] != "#" {
            mat[guard_x][guard_y] = "X";
            (guard_x, guard_y) = next;
        } else {
            mode = next_mode(&mode);
        }
    }

    Some(1 + count_x((&mat)))
}

fn next_move(x: usize, y: usize, mode: &Mode) -> (usize, usize) {
    match mode {
        Mode::North => (x - 1, y),
        Mode::East =>(x, y + 1),
        Mode::South =>(x + 1, y),
        Mode::West =>(x, y - 1),
    }
}

fn count_x(mat: &[Vec<&str>]) -> usize {
    mat.iter()
        .flat_map(|l| l.iter())
        .filter(|s| **s == "X")
        .count()
}

fn guard(mat: &[Vec<&str>]) -> (usize, usize) {
    let (height, width) = matrix_size(&mat);
    for x in 0..height {
        for y in 0..width {
            if mat[x][y] == "^" {
                return (x,y);
            }
        }
    }
    panic!("Guard (^) not found");
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
    fn test_count_x() {
        assert_eq!(count_x(&vec![
            vec![".", "X"],
            vec!["X", "."]]),
            2
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
