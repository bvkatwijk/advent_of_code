pub mod template;

pub enum Dir {
    North,
    East,
    South,
    West,
}

pub fn matrix(input: &str) -> Vec<Vec<&str>> {
    input.lines()
    .map(|l| l.split("").collect())
    .collect()
}
