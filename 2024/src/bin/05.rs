advent_of_code::solution!(5);

#[derive(Debug)]
pub struct Rule {
    pub left: usize,
    pub right: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut split_input = input.split("\n\n");

    let rules: Vec<Rule> = split_input.next().unwrap()
        .lines()
        .map(|l| l.split("|"))
        .map(|mut split| Rule { left: split.next().map(str::parse::<usize>).unwrap().unwrap(), right: split.next().map(str::parse::<usize>).unwrap().unwrap()})
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
        .flat_map(str::parse::<usize>)
        .collect()
}

fn is_valid(pages: &[usize], rules: &[Rule]) -> bool {
    let mut rules_remaining: Vec<&Rule> = rules.iter().collect();
    for page in pages {
        if rules_remaining.iter().any(|rule| rule.right == *page && pages.contains(&rule.left)) {
            return false;
        } else {
            rules_remaining = rules_remaining
                .into_iter()
                .filter(|r| r.left != *page)
                .collect();
        }
    }
    true
}

fn middle(input: Vec<usize>) -> usize {
    let size = input.len();
    input.into_iter()
        .nth(size /2)
        .unwrap()
}

// TODO implement
pub fn part_two(input: &str) -> Option<usize> {
    let mut split_input = input.split("\n\n");

    let rules: Vec<Rule> = split_input.next().unwrap()
        .lines()
        .map(|l| l.split("|"))
        .map(|mut split| Rule { left: split.next().map(str::parse::<usize>).unwrap().unwrap(), right: split.next().map(str::parse::<usize>).unwrap().unwrap()})
        .collect();

    Some(split_input
        .next()
        .unwrap()
        .lines()
        .map(parse_usize_vec)
        .filter(|l| !is_valid(l, &rules))
        .map(|l| corect_order(&l, &rules))
        .map(middle)
        .sum())
}

fn corect_order(l: &[usize], rules: &[Rule]) -> Vec<usize> {
    todo!()
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
    fn test_is_valid() {
        assert_eq!(is_valid(&vec![1,2], &vec![Rule{left: 1, right: 2}]), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
