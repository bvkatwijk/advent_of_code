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
        .map(|l| l.split(",").into_iter().collect::<Vec<&str>>())
        .filter(|l| is_valid(l, &rules))
        .map(middle)
        .sum())
}

// TODO implement
fn is_valid(l: &[usize], rules: &[Rule]) -> bool {
    true
}

fn middle(input: &str) -> usize {
    let elements: Vec<_> = input.split(",").into_iter().collect();
    let size = elements.len();
    elements.into_iter()
        .nth(size /2)
        .unwrap()
        .parse::<usize>()
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
    fn test_middle() {
        assert_eq!(middle("1,2,3"), 2);
        assert_eq!(middle("11,22,33"), 22);
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
