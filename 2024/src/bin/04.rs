advent_of_code::solution!(4);

const XMAS: &str = "XMAS";

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        count_horizontal(input)
            + count_horizontal(&rotate(input))
            + count_horizontal(&diag(input))
            + count_horizontal(&diag(&rotate(input))),
    )
}

pub fn flip_horizontal(input: &str) -> String {
    input
        .lines()
        .map(|l| l.chars().rev().collect::<String>() + "\n")
        .collect()
}

// both xmas and reverse (samx)
pub fn count_horizontal(input: &str) -> usize {
    let rev: String = XMAS.chars().rev().collect::<String>();
    input
        .lines()
        .map(|l| l.split(XMAS).count() - 1 + l.split(rev.as_str()).count() - 1)
        .sum()
}

pub fn rotate(input: &str) -> String {
    let size = input.lines().nth(0).unwrap().len();
    let mut strs: Vec<Vec<String>> = vec![vec![String::new(); size]; size];
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            strs[x][y] = c.to_string();
        })
    });
    strs.iter()
        .map(|v| v.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn diag(input: &str) -> String {
    let size = input.lines().nth(0).unwrap().len();
    let diags = (size * 2) - 1;
    let mut strs: Vec<Vec<String>> = vec![vec![String::new(); size]; diags];

    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            strs[x + y][y] = c.to_string();
        })
    });
    strs.iter()
        .map(|v| v.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(count_xmas_cross_all(input))
}

pub fn count_xmas_cross_all(input: &str) -> usize {
    count_xmas_cross(input)
        + count_xmas_cross(&flip_horizontal(input))
        + count_xmas_cross(&rotate(input))
        + count_xmas_cross(&flip_horizontal(&rotate(input)))
}

pub fn count_xmas_cross(input: &str) -> usize {
    let size = input.lines().nth(0).unwrap().len();
    let mut strs: Vec<Vec<char>> = vec![vec![' '; size]; size];
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            strs[x][y] = c;
        })
    });
    input
        .lines()
        .enumerate()
        .take(size - 2)
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .take(size - 2)
                .filter(|(x, _c)| has_xmas_cross(&strs, *x, y))
                .count()
        })
        .sum()
}

pub fn has_xmas_cross(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    input[x][y] == 'M'
        && input[x][y + 2] == 'M'
        && input[x + 1][y + 1] == 'A'
        && input[x + 2][y] == 'S'
        && input[x + 2][y + 2] == 'S'
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
    fn test_has_xmas_cross() {
        assert_eq!(
            has_xmas_cross(
                &vec![
                    vec!['M', '-', 'M'],
                    vec!['-', 'A', '-'],
                    vec!['S', '-', 'S']
                ],
                0,
                0
            ),
            true
        );
    }

    #[test]
    fn test_count_xmas_cross() {
        assert_eq!(count_xmas_cross("M.S\n.A.\nM.S"), 1);
    }

    #[test]
    fn test_stuff() {
        assert_eq!(count_xmas_cross(&flip_horizontal("S.M\n.A.\nS.M")), 1);
    }

    #[test]
    fn test_count_xmas_cross_all() {
        assert_eq!(count_xmas_cross_all("M.S.\n.A..\nM.S.\n...."), 1);
        assert_eq!(count_xmas_cross_all("S.M.\n.A..\nS.M.\n...."), 1);
        assert_eq!(count_xmas_cross_all("M.M.\n.A..\nS.S.\n...."), 1);
        assert_eq!(count_xmas_cross_all("MMMM\n.AA.\nSSSS\n...."), 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
