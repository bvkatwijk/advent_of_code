advent_of_code::solution!(3);

#[derive(PartialEq)]
pub enum Mul {
    M,
    U,
    L,
    Open,
    Num,
    Comma,
    Close
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(muls(input))
}

pub fn muls(input: &str) -> u32 {
    let mut total = 0;

    let mut prev: Mul = Mul::Close;
    let mut one: String = "".to_string();
    let mut two: String = "".to_string();
    // iterate over chars, if next is allowed, set next allowed,
    // if mul complete => parse, multiply, store

    for c in input.chars() {
        match c {
            'm' if prev == Mul::Close => prev = Mul::M,
            'u' if prev == Mul::M => prev = Mul::U,
            'l' if prev == Mul::U => prev = Mul::L,
            '(' if prev == Mul::L => prev = Mul::Open,
            '0'..='9' if prev == Mul::Open => one.push(c),
            ',' if prev == Mul::Open && one != "" => prev = Mul::Comma,
            '0'..='9' if prev == Mul::Comma => two.push(c),
            ')' if prev == Mul::Comma && two != "" => {
                total += one.parse::<u32>().unwrap() * two.parse::<u32>().unwrap();
                prev = Mul::Close;
                one = "".to_string();
                two = "".to_string();
            }
            _ => {
                // println!("mismatch, resetting: {c}");
                prev = Mul::Close;
                one = "".to_string();
                two = "".to_string();
            }
        }
    }
    total
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.split("do()") // split into do() segments
        .map(|seg| seg.split("don't()").into_iter().nth(0).unwrap()) // ignore everything after don't()
        .map(|s| muls(s)) // interpret muls
        .sum()) // sum partials
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("don't()mul(1,2)"), Some(0));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
    // 89889357 incorrect
}
