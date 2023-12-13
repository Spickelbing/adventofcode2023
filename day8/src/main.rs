use eyre::{anyhow, Report};
use lazy_regex::regex;
use num::Integer;
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

    let mut network: HashMap<String, (String, String)> = HashMap::new();
    let mut start_nodes = HashSet::<String>::new();
    let mut destination_nodes = HashSet::<String>::new();

    for line in input.lines().skip(2) {
        let node_regex = regex!(r"(\w{3}) = \((\w{3}), (\w{3})\)");
        let captures = node_regex.captures(line).unwrap();
        let node = captures[1].to_string();

        network.insert(
            node.clone(),
            (captures[2].to_string(), captures[3].to_string()),
        );

        if node.ends_with('A') {
            start_nodes.insert(node);
        } else if node.ends_with('Z') {
            destination_nodes.insert(node);
        }
    }

    let num_steps_from_start_to_dest: Vec<(&String, usize, &String)> = start_nodes
        .iter()
        .map(|start_node| {
            let mut num_steps = 0;
            let mut current_node = start_node;
            for direction in directions.iter().cycle() {
                if num_steps % directions.len() == 0 && destination_nodes.contains(current_node) {
                    break;
                }
                num_steps += 1;

                let (left_node, right_node) = &network[current_node];
                current_node = match direction {
                    Direction::Left => left_node,
                    Direction::Right => right_node,
                };
            }

            (start_node, num_steps, current_node)
        })
        .collect();

    let (_, num_steps_task_1, _) = num_steps_from_start_to_dest
        .iter()
        .find(|&(start_node, _, destination_node)| {
            start_node == &"AAA" && destination_node == &"ZZZ"
        })
        .unwrap();
    println!("Task 1: {num_steps_task_1}");

    // This works because the input was generated such that once a destination node is reached,
    // there is a loop in the network with regard to the list of directions.
    // That same destination node is reached again exactly on following the last direction
    // after some number of iterations through the full list of directions.
    // I feel this task is a bit wacky because solving it requires knowledge of this restriction
    // and it is only vaguely implied in the task description.
    let (_, mut lcm, _) = num_steps_from_start_to_dest[0];
    for (_, num_steps, _) in num_steps_from_start_to_dest.iter().skip(1) {
        lcm = lcm.lcm(num_steps);
    }

    println!("Task 2: {lcm}");

    Ok(())
}
