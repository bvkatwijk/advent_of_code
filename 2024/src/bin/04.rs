advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let xmas = "XMAS";
    Some(
        count_horizontal(input)
            + count_horizontal(rotate(input))
            + count_horizontal(diag(input))
            + count_horizontal(rotate(diag(input)))
    )
    // TODO how to rotate ??
    // TODO diag -> how to ??
}

// both xmas and reverse (samx)
pub fn count_horizontal(input: &str) -> usize {
    input.lines()
        .map(|l| l.split("XMAS").count() - 1 + l.split("SAMX").count() - 1)
        .sum()
}

pub fn rotate(input: &str) -> &str {
    ""
}

pub fn diag(input: &str) -> &str {
    ""
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
