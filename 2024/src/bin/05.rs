advent_of_code::solution!(5);

pub struct Rule {
    pub left: String,
    pub right: String,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut split_input = input.split("\n\n");

    let rules: Vec<Rule> = split_input.next().unwrap()
        .lines()
        .map(|l| l.split("|"))
        .map(|mut split| Rule { left: split.next().unwrap().to_string(), right: split.next().unwrap().to_string()})
        .collect();

    Some(split_input
        .next()
        .unwrap()
        .lines()
        .map(parse_usize_vec)
        .filter(|l| is_valid(l, &rules))
        .map(middle)
        .sum())
}

fn parse_usize_vec(input: &str) -> Vec<usize> {
    input.split(",")
        .into_iter()
        .flat_map(str::parse::<usize>) // Do we even care that they are numbers? maybe could skip this
        .collect()
}

// TODO implement
fn is_valid(l: &[usize], rules: &[Rule]) -> bool {
    true
}

fn middle(input: Vec<usize>) -> usize {
    let size = input.len();
    input.into_iter()
        .nth(size /2)
        .unwrap()
}

// TODO implement
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_usize_vec() {
        assert_eq!(parse_usize_vec("1,2"), vec![1, 2]);
    }

    #[test]
    fn test_middle() {
        assert_eq!(middle(vec![1,2,3]), 2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
