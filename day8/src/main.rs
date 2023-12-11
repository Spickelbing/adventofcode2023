use eyre::{anyhow, Report};
use lazy_regex::regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

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
    let mut start_node_indices = HashSet::<usize>::new();
    let mut destination_node_indices = HashSet::<usize>::new();
    let mut next_index_to_assign = 0;

    for line in input.lines().skip(2) {
        let (node, _) = line.split_once(" = ").unwrap();
        node_to_index_map.insert(node.to_string(), next_index_to_assign);

        if node.ends_with('A') {
            start_node_indices.insert(next_index_to_assign);
        } else if node.ends_with('Z') {
            destination_node_indices.insert(next_index_to_assign);
        }

        next_index_to_assign += 1;
    }

    let nodes: Vec<(usize, usize)> = input
        .lines()
        .skip(2)
        .map(|line| {
            let node_regex = regex!(r"^\w{3} = \((\w{3}), (\w{3})\)$");
            let captures = node_regex.captures(line).unwrap();

            let left_node_index = node_to_index_map[&captures[1]];
            let right_node_index = node_to_index_map[&captures[2]];

            (left_node_index, right_node_index)
        })
        .collect();

    // task 1
    {
        let destination_node_index = node_to_index_map["ZZZ"];
        let mut current_node_index: usize = node_to_index_map["AAA"];

        let mut num_steps: usize = 0;
        for direction in directions.iter().cycle() {
            if current_node_index == destination_node_index {
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
    };

    // task 2
    {
        let mut current_node_indices: HashSet<usize> = start_node_indices.clone();
        let mut next_node_indices = HashSet::<usize>::with_capacity(current_node_indices.len());
        let mut num_steps: usize = 0;

        for direction in directions.iter().cycle() {
            if current_node_indices.is_subset(&destination_node_indices) {
                break;
            }
            num_steps += 1;

            for &node_index in &current_node_indices {
                let (left_node_index, right_node_index) = nodes[node_index];
                match direction {
                    Direction::Left => next_node_indices.insert(left_node_index),
                    Direction::Right => next_node_indices.insert(right_node_index),
                };
            }

            let mut tmp = current_node_indices;
            tmp.clear();
            current_node_indices = next_node_indices;
            next_node_indices = tmp;
        }

        println!("Task 2: {num_steps}");
    }

    Ok(())
}
