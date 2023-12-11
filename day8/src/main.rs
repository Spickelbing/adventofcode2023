use eyre::{anyhow, Report};
use lazy_regex::regex;
use std::{collections::HashMap, fs};

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Report;
    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!("invalid character")),
        }
    }
}

fn main() -> Result<(), Report> {
    let input = fs::read_to_string("input")?;

    let directions: Vec<_> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .flat_map(Direction::try_from)
        .collect();

    let mut node_to_index_map: HashMap<String, usize> = HashMap::new();
    let mut next_index_to_assign = 0;
    for line in input.lines().skip(2) {
        let (node, _) = line.split_once(" = ").unwrap();
        node_to_index_map.insert(node.to_string(), next_index_to_assign);
        next_index_to_assign += 1;
    }

    let mut nodes: Vec<(usize, usize)> = Vec::new();
    for line in input.lines().skip(2) {
        let node_regex = regex!(r"^\w{3} = \((\w{3}), (\w{3})\)$");
        let captures = node_regex.captures(line).unwrap();

        let left_node_index = node_to_index_map[&captures[1]];
        let right_node_index = node_to_index_map[&captures[2]];

        nodes.push((left_node_index, right_node_index));
    }

    let last_node_index = node_to_index_map["ZZZ"];
    let mut num_steps: usize = 0;
    let mut current_node_index: usize = node_to_index_map["AAA"];
    for direction in directions.iter().cycle() {
        if current_node_index == last_node_index {
            break;
        }
        num_steps += 1;

        let (left_node_index, right_node_index) = nodes[current_node_index];
        current_node_index = match direction {
            Direction::Left => left_node_index,
            Direction::Right => right_node_index,
        };
    }

    println!("Task 1: {num_steps}");

    Ok(())
}
