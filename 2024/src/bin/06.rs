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
        let _next_value = mat[next.0][next.1];
        if mat[next.0][next.1] != "#" {
            mat[guard_x][guard_y] = "X";
            (guard_x, guard_y) = next;
        } else {
            mode = next_mode(&mode);
        }
    }

    Some(1 + count_x(&mat))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut mat = matrix(input);
    let (height, width) = matrix_size(&mat);
    let mut mode = Mode::North;
    let (mut guard_x, mut guard_y) = guard(&mat);

    // detect loop: solve, guard edge => no loop, guard start position = loop.

    // place single obstacle and detect loop
    // brute force? lets try
    // i could also only consider places where the guard visits... more efficient lets see how it performs initially
    while guard_x != 0 && guard_x != height - 1 && guard_y != 0 && guard_y != width - 1 {
        let next = next_move(guard_x, guard_y, &mode);
        let _next_value = mat[next.0][next.1];
        if mat[next.0][next.1] != "#" {
            mat[guard_x][guard_y] = "X";
            (guard_x, guard_y) = next;
        } else {
            mode = next_mode(&mode);
        }
    }

    Some(0)

    // Some(mat.iter()
    //     .enumerate()
    //     .flat_map(|(x, line)| line.iter()
    //         .enumerate()
    //         .map(|(y, element)| replace_and_detect_loop(&mat, x, y, guard_x, guard_y)))
    //     .filter())
}


fn replace_and_detect_loop(mat: &[Vec<&str>], obstacle_x: usize, obstacle_y: usize, guard_x: usize, guard_y: usize) -> bool {
    detect_loop(add_obstacle_at(mat, obstacle_x, obstacle_y), guard_x, guard_y)
}

fn add_obstacle_at<'a>(mat: &'a [Vec<&str>], _obstacle_x: usize, _obstacle_y: usize) -> &'a[Vec<&'a str>] {
    mat
}

fn detect_loop(_mat: &[Vec<&str>], _x: usize, _y: usize) -> bool {
    false
}

fn detect_remaining_loop(mat: &mut [Vec<&str>], _guard_start_x: usize, _guard_start_y: usize, _guard_x: usize, _guard_y: usize, _mode: &Mode) -> bool {
    let (height, width) = matrix_size(&mat);
    let mut mode = Mode::North;
    let (mut guard_x, mut guard_y) = guard(&mat);

    while guard_x != 0 && guard_x != height - 1 && guard_y != 0 && guard_y != width - 1 {
        let next = next_move(guard_x, guard_y, &mode);
        let _next_value = mat[next.0][next.1];
        if mat[next.0][next.1] != "#" {
            mat[guard_x][guard_y] = "X";
            (guard_x, guard_y) = next;
        } else {
            mode = next_mode(&mode);
        }
    }
    true
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
    fn test_add_obstacle_at() {
        assert_eq!(add_obstacle_at(&vec![
            vec![".", "."],
            vec!["^", "."]], 0, 0),
            vec![
            vec!["X", "."],
            vec!["^", "."]]
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
