use aoc_1;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/02_01_example.txt";

fn aoc_2_1(input: &str) -> u32 {
    return 1;
    // return aoc_1::file_lines(input)
    //     .map(|l| l.unwrap())
    //     .sum();
}



#[cfg(test)]
mod tests{
    use crate::aoc_2::{aoc_2_1, EXAMPLE_01};

    #[test]
    fn aoc_1_2_example_test() {
        assert_eq!(1, aoc_2_1(EXAMPLE_01));
    }
}