pub mod template;

pub fn matrix(input: &str) -> Vec<Vec<&str>> {
    input.lines()
    .map(|l| l.split("").collect())
    .collect()
}
