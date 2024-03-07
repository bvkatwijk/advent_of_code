use crate::helper;

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
        .map(Pipe::parse_line)
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
        .find(|(_, value)| Pipe::Start.eq(value))
        .map(|(index, _)| index)
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Pipe {
    Vertical,
    Horizontal,
    CornerNE,
    CornerNw,
    CornerSe,
    CornerSw,
    Ground,
    Start   
}

impl Pipe {
    fn parse_line(l : String) -> Vec<Pipe> {
        l.split("")
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(Pipe::parse)
            .collect()
    }

    fn parse(c: &str) -> Pipe {
        match c {
            "|" => Pipe::Vertical,
            "-" => Pipe::Horizontal,
            "L" => Pipe::CornerNE,
            "J" => Pipe::CornerNw,
            "7" => Pipe::CornerSw,
            "F" => Pipe::CornerSe,
            "." => Pipe::Ground,
            "S" => Pipe::Start,
            _ => panic!("unknown: {}", c)
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

    // #[test]
    // fn aoc_10_1_test() {
    //     assert_eq!(114, aoc_10_1(EXAMPLE_01));
    //     // assert_eq!(1725987467, aoc_10_1(INPUT));
    // }

    #[test]
    fn find_start_test() {
        assert_eq!((1, 1), find_start(&create_grid(EXAMPLE_01)));
        assert_eq!((2, 0), find_start(&create_grid(EXAMPLE_02)));
    }

    #[test]
    fn pipe_parse() {
        assert_eq!(
            vec![Pipe::Ground, Pipe::Start, Pipe::Horizontal, Pipe::CornerSw, Pipe::Ground],
            Pipe::parse_line(".S-7.".to_owned())
        );
    }

    // #[test]
    // fn aoc_10_2_test() {
    //     assert_eq!(2, aoc_10_2(EXAMPLE_01));
    //     assert_eq!(971, aoc_10_2(INPUT));
    // }
}
