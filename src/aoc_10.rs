use crate::helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_10/example_1.txt";
#[allow(dead_code)]
const INPUT: &str = "./src/aoc_10/input.txt";

// #[allow(dead_code)]
// fn aoc_10_1(path: &str) -> i64 {
//     let grid: Vec<Vec<String>> = helper::file_lines(path)
//         .map(Result::unwrap)
//         .flat_map(chars)
//         .collect();
//     0
// }

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
