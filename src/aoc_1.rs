use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_1/01_01_example.txt";
#[allow(dead_code)]
const EXAMPLE_02: &str = "./src/aoc_1/01_02_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_1/01.txt";

#[allow(dead_code)]
fn aoc_1_1(input: &str) -> u32 {
    helper::file_lines(input)
        .map(|l| l.unwrap())
        .map(|s| word_value_01(&s))
        .sum()
}

#[allow(dead_code)]
fn aoc_1_2(input: &str) -> u32 {
    helper::file_lines(input)
        .map(|l| l.unwrap())
        .map(|s| word_value_02(&s))
        .sum()
}

// Converts string to appended first and last digit, including digit words
// e.g. "a1btwo" -> 12
fn word_value_02(input: &str) -> u32 {
    let mut both = String::from("");
    let lr = &replace_number_words_lr(input);
    let rl = &replace_number_words_rl(input);
    both.push_str(lr);
    both.push_str(rl);
    word_value_01(&both)
}

// Converts string to appended first and last digit
// e.g. "a1b2" -> 12
fn word_value_01(input: &str) -> u32 {
    let v = digits(input);
    (10 * v.first().unwrap()) + v.last().unwrap()
}

// Replace number words with digit, from left to right
fn replace_number_words_lr(input: &str) -> String {
    let mut str = String::from("");
    for c in input.chars() {
        str.push(c);
        str = replace_number_words_smallest_first(&str.as_str());
    }
    str
}

// Replace number words with digit, from right to left
fn replace_number_words_rl(input: &str) -> String {
    let mut str: String = String::from("");
    for c in input.chars().rev() {
        str.insert(0, c);
        str = replace_number_words_smallest_first(&str.as_str());
    }
    str
}

// Replace number words with digit, from small to large
fn replace_number_words_smallest_first(input: &str) -> String {
    let all_digits: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut str = input.to_string();
    for i in all_digits {
        str = replace_number(&str, i);
    }
    str
}

// Convert string to vec of digits
// e.g. "1a2b3" -> 123
fn digits(s: &str) -> Vec<u32> {
    s.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

// Replace digit word in a string with the corresponding digit
// e.g. "atwo" -> "a2"
fn replace_number(input: &str, target: u8) -> String {
    let target_str = number_to_word(&target).unwrap();
    input.replace(target_str, &word_to_number(target_str).unwrap().to_string())
}

// Map maybe digit word to Option digit
// e.g. "one" -> Some(1)
fn word_to_number(input: &str) -> Option<u8> {
    match input {
        "one"  => Some(1),
        "two"  => Some(2),
        "three"  => Some(3),
        "four"  => Some(4),
        "five"  => Some(5),
        "six"  => Some(6),
        "seven"  => Some(7),
        "eight"  => Some(8),
        "nine"  => Some(9),
        _      => None,
    }
}

fn number_to_word(input: &u8) -> Option<&str> {
    match input {
        1 => Some("one"),
        2 => Some("two"),
        3 => Some("three"),
        4 => Some("four"),
        5 => Some("five"),
        6 => Some("six"),
        7 => Some("seven"),
        8 => Some("eight"),
        9 => Some("nine"),
        _      => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1_1_result() {
        assert_eq!(142, aoc_1_1(EXAMPLE_01));
        assert_eq!(55712, aoc_1_1(ACTUAL));
    }

    #[test]
    fn aoc_1_2_result() {
        assert_eq!(281, aoc_1_2(EXAMPLE_02));
        assert_eq!(55413, aoc_1_2(ACTUAL));
    }

    #[test]
    fn digits_test() {
        assert_eq!(1, digits("1")[0]);
        assert_eq!(1, digits("a1")[0]);
        assert_eq!(1, digits("a1b")[0]);
        assert_eq!(1, digits("1b")[0]);
    }

    #[test]
    fn word_to_number_test() {
        assert_eq!(None, word_to_number("a"));
        assert_eq!(None, word_to_number("1"));
        assert_eq!(Some(1), word_to_number("one"));
        assert_eq!(Some(2), word_to_number("two"));
    }

    #[test]
    fn replace_number_test() {
        assert_eq!("a", replace_number("a", 1));
        assert_eq!("a", replace_number("a", 2));

        assert_eq!("on", replace_number("on", 1));
        assert_eq!("on", replace_number("on", 2));

        assert_eq!("1", replace_number("one", 1));
        assert_eq!("2", replace_number("two", 2));
        assert_eq!("3", replace_number("three", 3));
        assert_eq!("4", replace_number("four", 4));
        assert_eq!("5", replace_number("five", 5));
        assert_eq!("6", replace_number("six", 6));
        assert_eq!("7", replace_number("seven", 7));
        assert_eq!("8", replace_number("eight", 8));
        assert_eq!("9", replace_number("nine", 9));

        assert_eq!("11", replace_number("1one", 1));
        assert_eq!("1one", replace_number("1one", 2));
        assert_eq!("12", replace_number("1two", 2));

        assert_eq!("11", replace_number("one1", 1));
        assert_eq!("1two", replace_number("onetwo", 1));
        assert_eq!("12", replace_number(&replace_number("onetwo", 1), 2));
    }

    #[test]
    fn replace_number_words_lr_test() {
        assert_eq!("1", replace_number_words_lr("one"));
        assert_eq!("12", replace_number_words_lr("onetwo"));
        assert_eq!("219", replace_number_words_lr("two1nine"));
        assert_eq!("8wo", replace_number_words_lr("eightwo"));
    }

    #[test]
    fn replace_number_words_rl_test() {
        assert_eq!("1", replace_number_words_rl("one"));
        assert_eq!("12", replace_number_words_rl("onetwo"));
        assert_eq!("219", replace_number_words_rl("two1nine"));
        assert_eq!("eigh2", replace_number_words_rl("eightwo"));
    }

    #[test]
    fn word_value_01_test() {
        assert_eq!(12, word_value_01("1abc2"));
        assert_eq!(38, word_value_01("pqr3stu8vwx"));
        assert_eq!(15, word_value_01("a1b2c3d4e5f"));
        assert_eq!(77, word_value_01("treb7uchet"));
    }

    #[test]
    fn word_value_02_test() {
        // Given examples
        assert_eq!(29, word_value_02("two1nine"));
        assert_eq!(83, word_value_02("eightwothree"));
        assert_eq!(13, word_value_02("abcone2threexyz"));
        assert_eq!(24, word_value_02("xtwone3four"));
        assert_eq!(42, word_value_02("4nineeightseven2"));
        assert_eq!(14, word_value_02("zoneight234"));
        assert_eq!(76, word_value_02("7pqrstsixteen"));

        // Custom examples  
        assert_eq!(77, word_value_02("seven"));
        assert_eq!(77, word_value_02("7"));
        assert_eq!(82, word_value_02("eightwo"));

        // Include first part examples
        assert_eq!(12, word_value_02("1abc2"));
        assert_eq!(38, word_value_02("pqr3stu8vwx"));
        assert_eq!(15, word_value_02("a1b2c3d4e5f"));
        assert_eq!(77, word_value_02("treb7uchet"));
    }
}
