advent_of_code::solution!(4);

const XMAS: &str = "XMAS";

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        count_xmas(input)
            + count_xmas(&flip_horizontal(input))
            + count_xmas(&rotate(input))
            + count_xmas(&rotate(&flip_horizontal(input)))
            + count_xmas(&diag(input))
            + count_xmas(&diag(&flip_horizontal(input)))
            + count_xmas(&diag(&rotate(input)))
            + count_xmas(&diag(&rotate(&flip_horizontal(input))))
    )
}

pub fn flip_horizontal(input: &str) -> String {
    input.lines()
        .rev()
        .collect()
}

// both xmas and reverse (samx)
pub fn count_xmas(input: &str) -> usize {
    input.lines()
        .map(|l| l.split(XMAS).count() - 1)
        .sum()
}

pub fn rotate(input: &str) -> String {
    let size = input.lines().nth(0).unwrap().len();
    let mut strs : Vec<Vec<String>> = vec![vec![String::new(); size]; size];
    input.lines()
        .enumerate()
        .for_each(|(y, l)| l.chars().enumerate().for_each(|(x, c)| {
            strs[x][y] = c.to_string();
    }));
    strs
        .iter()
        .map(|v| v.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn diag(input: &str) -> String {
    let size = input.lines().nth(0).unwrap().len();
    let diags = (size*2)-1;
    let mut strs : Vec<Vec<String>> = vec![vec![String::new(); size]; diags];

    input.lines()
        .enumerate()
        .for_each(|(y, l)| l.chars().enumerate().for_each(|(x, c)| {
            strs[x+y][y] = c.to_string();
    }));
    strs
        .iter()
        .map(|v| v.join(""))
        .collect::<Vec<String>>()
        .join("\n")
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
        assert_eq!(result, Some(9));
    }
}
