use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_3/03_01_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_3/03_01.txt";

#[allow(dead_code)]
fn aoc_3_1(path: &str) {
    let matrix: Vec<String> = matrix(path);
    let x = matrix.iter().map(String::as_str).collect();
    let _numbers = numbers_in_matrix(x);
}

// Returns a Vec of Numbers from the given matrix
fn numbers_in_matrix(matrix: Vec<&str>) -> Vec<Number> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(index, value)| numbers_in_line(value, index as u8))
        .collect()
}

// Returns a Vec of Numbers from the given line
fn numbers_in_line(input: &str, line: u8) -> Vec<Number> {
    input
        .split(|c: char| !c.is_digit(10))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .map(|i| {
            let x = input.find(&i.to_string()).unwrap();
            let len = i.to_string().len();
            Number {
                value: i,
                line: line as usize,
                start_x: x,
                end_x: x + (len - 1),
            }
        })
        .collect()
}

#[allow(dead_code)]
// Returns vector of adjacent symbols
fn adjacent_symbols(grid: &Vec<String>, number: Number) -> Vec<&str> {
    let line_above: usize = match number.line {
        0 => 0,
        _ => number.line - 1,
    };
    println!("line_above {}", line_above);
    let line_below = std::cmp::min(grid.len() - 1, number.line + 2);
    println!("line_below {}", line_below);
    let x_start = match number.start_x {
        0 => 0,
        _ => number.start_x - 1,
    };
    println!("x_start {}", x_start);
    let x_end = std::cmp::min(grid[0].len() - 1, number.end_x + 2);
    println!("x_end {}", x_end);
    grid[line_above..line_below + 1]
        .iter()
        .map(|s| &s[x_start..number.end_x])
        .map(|s| {
            println!("s: {}", &s);
            return s;
        })
        .collect()
}

// #[allow(dead_code)]
// fn aoc_3_2(path: &str) {

// }

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: u32,
    line: usize,
    start_x: usize,
    end_x: usize,
}

fn matrix(path: &str) -> Vec<String> {
    helper::file_lines(path)
        .map(|l: Result<String, std::io::Error>| l.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE_1: &str = "467..114..";
    const LINE_10: &str = ".664.598..";
    const NO_114: Number = Number {
        value: 114,
        line: 0,
        start_x: 5,
        end_x: 7,
    };
    const NO_467: Number = Number {
        value: 467,
        line: 0,
        start_x: 0,
        end_x: 2,
    };
    const NO_664: Number = Number {
        value: 664,
        line: 1,
        start_x: 1,
        end_x: 3,
    };
    const NO_598: Number = Number {
        value: 598,
        line: 1,
        start_x: 5,
        end_x: 7,
    };

    #[test]
    fn matrix_test() {
        assert_eq!(LINE_1, matrix(EXAMPLE_01)[0]);
    }

    #[test]
    fn numbers_in_matrix_test() {
        let result = numbers_in_matrix(vec![LINE_1, LINE_10]);
        assert_eq!(NO_467, result[0]);
        assert_eq!(NO_114, result[1]);
    }

    #[test]
    fn numbers_in_line_test() {
        let line_0 = numbers_in_line(LINE_1, 0);
        let line_9 = numbers_in_line(LINE_10, 1);
        assert_eq!(NO_467, line_0[0]);
        assert_eq!(NO_114, line_0[1]);
        assert_eq!(NO_664, line_9[0]);
        assert_eq!(NO_598, line_9[1]);
    }

    #[test]
    fn adjacent_symbols_test() {
        let matrix = vec![LINE_1.to_string()];
        assert_eq!(vec!["."], adjacent_symbols(&matrix, NO_467));
        assert_eq!(vec![".", "."], adjacent_symbols(&matrix, NO_114));
    }
}
