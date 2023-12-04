use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_4/04_01_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_4/04_01.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> u32 {
    // helper::file_lines(&path)
    //     .map(|l| l.unwrap())
    //     .map(|l| winning_numbers(l))
    //     .map(|s| s[&0])
    0
}

fn winning_numbers(input: &str) -> Vec<&u8> {
    let split: Vec<&str> = input.split(":").collect();
    let split2: Vec<&str> = split[1].split("|").map(|s| s.trim()).collect();
    let winning: Vec<u8> = numbers(split2[0]);
    let actual: Vec<u8> = numbers(split2[1]);
    actual.iter()
        .filter(|i| winning.contains(i))
        .collect()
}

fn numbers(input: &str) -> Vec<u8> {
    input.split(" ").map(|s| s.parse::<u8>().unwrap()).collect()
}

// #[allow(dead_code)]
// fn aoc_4_2(path: &str) {
    
// }

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn matrix_test() {
        assert_eq!(13, aoc_4_1(EXAMPLE_01));
    }

    #[test]
    fn winning_numbers_test() {
        assert_eq!(vec![48, 83, 17, 86], winning_numbers("41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
    }
}
