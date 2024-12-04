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
    let iter = input.chars();

    let mut total = 0;

    let mut prev: Mul = Mul::Close;
    let mut toggle = true;
    let lit_do: &str = "do()";
    let mut count_do = 0;
    let lit_dont: &str = "don't()";
    let mut count_dont = 0;

    let mut one: String = "".to_string();
    let mut two: String = "".to_string();

    for c in iter {
        if toggle {
            match c {
                c if c == lit_dont.chars().nth(count_dont).unwrap() => {
                    println!("match progress don't: {c}");
                    if c == ')' {
                        println!("disabling!");
                        toggle = false;
                        reset(&mut prev, &mut one, &mut two, &mut count_do, &mut count_dont);
                    } else {
                        count_dont += 1
                    }
                }
                'm' if prev == Mul::Close => prev = Mul::M,
                'u' if prev == Mul::M => prev = Mul::U,
                'l' if prev == Mul::U => prev = Mul::L,
                '(' if prev == Mul::L => prev = Mul::Open,
                '0'..='9' if prev == Mul::Open => one.push(c),
                ',' if prev == Mul::Open && one != "" => prev = Mul::Comma,
                '0'..='9' if prev == Mul::Comma => two.push(c),
                ')' if prev == Mul::Comma && two != "" => {
                    total += one.parse::<u32>().unwrap() * two.parse::<u32>().unwrap();
                    reset(&mut prev, &mut one, &mut two, &mut count_do, &mut count_dont);
                }
                _ => {
                    reset(&mut prev, &mut one, &mut two, &mut count_do, &mut count_dont);
                }
            }
        } else {
            if c == lit_do.chars().nth(count_do).unwrap() {
                println!("match progress do: {c}");
                if c == ')' {
                    println!("enabling!");
                    toggle = true;
                    reset(&mut prev, &mut one, &mut two, &mut count_do, &mut count_dont);
                } else {
                    count_do += 1
                }
            }
        }
    }

    Some(total)
}

fn reset(prev: &mut Mul, one: &mut String, two: &mut String, count_do: &mut usize, count_dont: &mut usize) {
    *prev = Mul::Close;
    *one = "".to_string();
    *two = "".to_string();
    *count_do = 0;
    *count_dont = 0;
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
    // 89889357 incorrect
}
