use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_3/03_01_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_3/03_01.txt";

#[allow(dead_code)]
fn aoc_3_1(path: &str) {
    let matrix = matrix(path);
    let numbers = numbers_in_matrix(&matrix);
}

fn numbers_in_matrix(matrix: &Vec<Vec<String>>) -> Vec<Number> {
    matrix.iter()
        .enumerate()
        .flat_map(|(index, value)| numbers_in_line(value, index as u8))
        .collect()
}

fn numbers_in_line(input: &Vec<String>, line: u8) -> Vec<Number> {
    input
        .concat()   
        .split(|c: char| !c.is_digit(10))
        .filter(|s| !s.is_empty()) 
        .map(|s| s.parse::<u32>().unwrap())
        .map(|i| Number {
            value: i,
            line: line,
            start_x: 0,
            end_x: 2
        })
        .collect()
}

#[allow(dead_code)]
fn aoc_3_2(path: &str) {
    
}


#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: u32,
    line: u8,
    start_x: u8,
    end_x: u8
}

fn matrix(path: &str) -> Vec<Vec<String>> {
    helper::file_lines(path)
        .map(|l: Result<String, std::io::Error>| l.unwrap())
        .map(|l| line_to_vec(&l))
        .collect()
}

fn line_to_vec(input: &str) -> Vec<String> {
    input.chars()
        .map(|c| c.to_string())
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    const LINE_1: &str = "467..114..";
    const LINE_10: &str = ".664.598..";

    #[test]
    fn matrix_test() {
        assert_eq!("4", matrix(EXAMPLE_01)[0][0]);
        assert_eq!("6", matrix(EXAMPLE_01)[0][1]);
        assert_eq!("7", matrix(EXAMPLE_01)[0][2]);
        assert_eq!(".", matrix(EXAMPLE_01)[0][3]);
        assert_eq!("1", matrix(EXAMPLE_01)[0][6]);
        assert_eq!("4", matrix(EXAMPLE_01)[0][7]);
        assert_eq!("8", matrix(EXAMPLE_01)[9][7]);
    }

    #[test]
    fn line_to_vec_test() {
        assert_eq!("4", line_to_vec(LINE_1)[0]);
    }

    #[test]
    fn numbers_in_matrix_test() {
        let no_467 = Number {
            value: 467,
            line: 0,
            start_x: 0,
            end_x: 2
        };
        let no_114 = Number {
            value: 114,
            line: 0,
            start_x: 5,
            end_x: 7
        };
        let result = numbers_in_matrix(&vec![line_to_vec(LINE_1)]);
        assert_eq!(no_467, result[0]);
        assert_eq!(no_114, result[1]);
    }

    #[test]
    fn numbers_in_line_test() {
        let no_467 = Number {
            value: 467,
            line: 0,
            start_x: 0,
            end_x: 2
        };
        let no_114 = Number {
            value: 114,
            line: 0,
            start_x: 5,
            end_x: 7
        };
        let result = numbers_in_line(&line_to_vec(LINE_1), 0);
        assert_eq!(no_467, result[0]);
        assert_eq!(no_114, result[1]);
    }
}