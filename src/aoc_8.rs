use std::{collections::HashMap};

use crate::helper::{self, debug};

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_8/example_01.txt";
#[allow(dead_code)]
const EXAMPLE_02: &str = "./src/aoc_8/example_02.txt";
#[allow(dead_code)]
const EXAMPLE_03: &str = "./src/aoc_8/example_03.txt";
#[allow(dead_code)]
const INPUT: &str = "./src/aoc_8/input.txt";

#[allow(dead_code)]
fn aoc_8_1(path: &str) -> usize {
    let lines: Vec<String> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .collect();
    let instructions: Vec<Direction> = instructions(&lines[0]);
    let network = network(&lines[2..]);
    walk_network_01(&network, &instructions)
}

#[allow(dead_code)]
fn aoc_8_2(path: &str) -> usize {
    let lines: Vec<String> = helper::file_lines(path)
        .map(|l| l.unwrap())
        .collect();
    let instructions: Vec<Direction> = instructions(&lines[0]);
    let network = network(&lines[2..]);
    walk_network_02(&network, &instructions)
}

fn walk_network_01(network: &HashMap<String, Node>, instructions: &Vec<Direction>) -> usize {
    walk_network_for_node(network, instructions, &"AAA")
}

fn walk_network_for_node(network: &HashMap<String, Node>, instructions: &Vec<Direction>, start: &str) -> usize {
    let mut node = network.get(start).unwrap();
    let mut steps: usize = 0;
    while !node.name.eq("ZZZ") {
        let dir = &instructions[steps % instructions.len()];
        node = next_node(dir, node, &network);
        steps += 1;
    }
    steps
}

fn walk_network_for_nodeZ(network: &HashMap<String, Node>, instructions: &Vec<Direction>, start: &str) -> usize {
    let mut node = network.get(start).unwrap();
    let mut steps: usize = 0;
    while !node.name.ends_with("Z") {
        let dir = &instructions[steps % instructions.len()];
        node = next_node(dir, node, &network);
        steps += 1;
    }
    steps
}

fn next_node<'a>(dir: &Direction, current: &Node, network: &'a HashMap<String, Node>) -> &'a Node {
    let result = current.pick(dir);
    network.get(&result.to_string()).unwrap()
}

fn walk_network_02(network: &HashMap<String, Node>, instructions: &Vec<Direction>) -> usize {
    let nodes: Vec<&Node> = start_node(&network);
    debug(&nodes);
    nodes
        .iter()
        .map(|node| walk_network_for_nodeZ(network, instructions, &node.name))
        .fold(1, |a, b| lcm(&[a, b]))
}

fn start_node(network: &HashMap<String, Node>) -> Vec<&Node> {
    network.values().into_iter().filter(|n| n.name.ends_with("A")).collect()
}

// Copied this from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

// Copied this from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn instructions(lines: &str) -> Vec<Direction> {
    lines.chars()
        .map(|c| Direction::from(&c.to_string()))
        .map(|o| o.unwrap())
        .collect()
}

fn network(lines: &[String]) -> HashMap<String, Node> {
    let mut draw = HashMap::new(); 
        
    lines.iter()
        .map(|s| as_node(s))
        .for_each(|e| {
            draw.insert(e.name.to_string(), e);
        });

    draw
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
    fn pick(&self, dir: &Direction) -> &str {
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
        assert_eq!(6, aoc_8_1(EXAMPLE_02));
        assert_eq!(23147, aoc_8_1(INPUT));
    }

    #[test]
    fn aoc_8_2_test() {
        assert_eq!(6, aoc_8_2(EXAMPLE_03));
        assert_eq!(23147, aoc_8_2(INPUT));
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
        assert_eq!("BBB", node.pick(&Direction::Left));
        assert_eq!("CCC", node.pick(&Direction::Right));
    }

    #[test]
    fn instructions_test() {
        assert_eq!(vec![Direction::Left], instructions("L"));
        assert_eq!(vec![Direction::Left, Direction::Right], instructions("LR"));
    }
}
