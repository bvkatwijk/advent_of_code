use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_3/03_01_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_3/03_01.txt";

#[allow(dead_code)]
fn aoc_3_1(path: &str) {
    let matrix = matrix(path);

}

#[allow(dead_code)]
fn aoc_3_2(path: &str) {
    
}

fn matrix(path: &str) -> Vec<Vec<String>> {
    helper::file_lines(path)
        .map(|l| l.unwrap())
        .map(|l| line_to_vec(&l))
        .collect()
}

fn line_to_vec(input: &String) -> Vec<String> {
    input.split("")
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn matrix_test() {
        assert_eq!("a", matrix(EXAMPLE_01)[0][0]);
    }
}