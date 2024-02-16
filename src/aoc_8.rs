use crate::helper::{self};

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_8/example_01.txt";
#[allow(dead_code)]
const EXAMPLE_02: &str = "./src/aoc_8/example_02.txt";

#[allow(dead_code)]
fn aoc_8_1(path: &str) -> usize {
    let lines: Vec<String> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .collect();
    let instructions: &str = &lines[0];
    let network = network(&lines[2..]);

    let mut steps: usize = 0;
    loop {
        steps += 1;
        break;
    }

    steps
}

fn network(lines: &[String]) -> Vec<Node> {
    lines.iter()
        .map(|s| as_node(s))
        .collect()
}

fn as_node(s: &str) -> Node {
    let split: Vec<&str> = s.split("=")
        .map(|s| s.trim())
        .collect();
    let trimmed = split[1]
        .replace(&['(', ')'][..], "");
    let left_right: Vec<&str> = trimmed
        .split(",")
        .map(|s| s.trim())
        .collect();
    Node {
        name: split[0].to_owned(),
        left: left_right[0].to_owned(),
        right: left_right[1].to_owned(),
    }
}


#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right
}

impl Direction {
    fn from(str: &str) -> Option<Direction> {
        match str {
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None
        }
    }
}

#[derive(Debug, Hash, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String
}

impl Node {
    fn pick(&self, dir: Direction) -> &str {
        match dir {
            Direction::Left => &self.left,
            Direction::Right => &self.right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_8_1_test() {
        assert_eq!(2, aoc_8_1(EXAMPLE_01));
    }

    #[test]
    fn node_test() {
        assert_eq!(Node {
            name: "AAA".to_owned(),
            left: "BBB".to_owned(),
            right: "CCC".to_owned(),
        }, as_node("AAA = (BBB, CCC)"))
    }

    #[test]
    fn direction_from_test() {
        assert_eq!(Some(Direction::Left), Direction::from("L"));
        assert_eq!(Some(Direction::Right), Direction::from("R"));
        assert_eq!(None, Direction::from("Z"));
    }

    #[test]
    fn node_pick_test() {
        let node = Node {
            name: "AAA".to_owned(),
            left: "BBB".to_owned(),
            right: "CCC".to_owned(),
        };
        assert_eq!("BBB", node.pick(Direction::Left));
        assert_eq!("CCC", node.pick(Direction::Right));
    }
}
