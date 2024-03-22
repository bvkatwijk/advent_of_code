use crate::{direction::{self, Direction}, helper};

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_10/example_1.txt";
#[allow(dead_code)]
const EXAMPLE_02: &str = "./src/aoc_10/example_2.txt";
#[allow(dead_code)]
const INPUT: &str = "./src/aoc_10/input.txt";

#[allow(dead_code)]
fn aoc_10_1(path: &str) -> i64 {
    let grid: Vec<Vec<Pipe>> = create_grid(path);
    let start: (usize, usize) = find_start(&grid);
    0
}

fn create_grid(path: &str) -> Vec<Vec<Pipe>> {
    helper::file_lines(path)
        .map(Result::unwrap)
        .enumerate()
        .map(|(i, v)| Pipe::parse_line(i, &v))
        .collect()
}

fn find_start(grid: &Vec<Vec<Pipe>>) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(index, value)| vec_find_start(value).map(|it| (index, it)))
        .unwrap()
}

fn vec_find_start(vec: &Vec<Pipe>) -> Option<usize> {
    vec.iter()
        .enumerate()
        .find(|(_, value)| Kind::Start.eq(&value.kind))
        .map(|(index, _)| index)
}

fn path_length(grid: &Vec<Vec<Pipe>>) -> usize {
    let mut count: usize = 0;
    // let start: Pipe = find_start(grid);
    // let mut one_current: Pipe = 0;
    0
}

// Walk grid from position in direction
fn navigate<'a>(grid: &'a Vec<Vec<Pipe>>, pos: &Pipe, direction: Direction) -> &'a Pipe {
    let new_pos = walk((&pos.x, &pos.y), direction);
    &grid[new_pos.0][new_pos.1]
}

fn walk(pos: (&usize, &usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::North => (pos.0 - 1, *pos.1),
        Direction::East => (*pos.0, pos.1 + 1),
        Direction::South => (pos.0 + 1, *pos.1),
        Direction::West => (*pos.0, pos.1 - 1),
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Kind {
    Vertical,
    Horizontal,
    CornerNE,
    CornerNw,
    CornerSe,
    CornerSw,
    Ground,
    Start,
}

impl Kind {
    fn dirs(&self) -> Option<(Direction, Direction)> {
        match self {
            Kind::Vertical => Some((Direction::North, Direction::South)),
            Kind::Horizontal => Some((Direction::East, Direction::West)),
            Kind::CornerNE => Some((Direction::North, Direction::East)),
            Kind::CornerNw => Some((Direction::North, Direction::West)),
            Kind::CornerSe => Some((Direction::South, Direction::East)),
            Kind::CornerSw => Some((Direction::South, Direction::West)),
            Kind::Ground => None,
            Kind::Start => None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pipe {
    kind: Kind,
    x: usize,
    y: usize,
}

impl Pipe {
    fn parse_line(x: usize, l: &str) -> Vec<Pipe> {
        l.split("")
            .into_iter()
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(index, value)| Pipe {
                kind: Pipe::parse(value),
                x: x,
                y: index,
            })
            .collect()
    }

    fn parse(c: &str) -> Kind {
        match c {
            "|" => Kind::Vertical,
            "-" => Kind::Horizontal,
            "L" => Kind::CornerNE,
            "J" => Kind::CornerNw,
            "7" => Kind::CornerSw,
            "F" => Kind::CornerSe,
            "." => Kind::Ground,
            "S" => Kind::Start,
            _ => panic!("unknown: {}", c),
        }
    }
}

// fn grid(lines: Vec<String>) -> Vec<Vec<char>> {

// }

// #[allow(dead_code)]
// fn aoc_10_2(path: &str) -> i64 {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_10_1_test() {
        // assert_eq!(4, aoc_10_1(EXAMPLE_01));
        // assert_eq!(1725987467, aoc_10_1(INPUT));
    }

    #[test]
    fn find_start_test() {
        assert_eq!((1, 1), find_start(&create_grid(EXAMPLE_01)));
        assert_eq!((2, 0), find_start(&create_grid(EXAMPLE_02)));
    }

    #[test]
    fn navigate_test() {
        let grid = create_grid(EXAMPLE_01);
        let p_12 = &grid[1][2];
        assert_eq!(p_12, navigate(&grid, &grid[1][1], Direction::East));
    }

    #[test]
    fn walk_test() {
        assert_eq!((0,1), walk((&1,&1), Direction::North));
        assert_eq!((1,2), walk((&1,&1), Direction::East));
        assert_eq!((2,1), walk((&1,&1), Direction::South));
        assert_eq!((1,0), walk((&1,&1), Direction::West));
    }

    #[test]
    fn pipe_parse() {
        assert_eq!(
            vec![Pipe {
                kind: Kind::Start,
                x: 0,
                y: 0
            }],
            Pipe::parse_line(0, "S")
        );
        assert_eq!(
            vec![
                Pipe {
                    kind: Kind::Ground,
                    x: 1,
                    y: 0
                },
                Pipe {
                    kind: Kind::Start,
                    x: 1,
                    y: 1
                },
                Pipe {
                    kind: Kind::Horizontal,
                    x: 1,
                    y: 2
                },
                Pipe {
                    kind: Kind::CornerSw,
                    x: 1,
                    y: 3
                },
                Pipe {
                    kind: Kind::Ground,
                    x: 1,
                    y: 4
                }
            ],
            Pipe::parse_line(1, ".S-7.")
        );
    }

    // #[test]
    // fn aoc_10_2_test() {
    //     assert_eq!(2, aoc_10_2(EXAMPLE_01));
    //     assert_eq!(971, aoc_10_2(INPUT));
    // }
}
