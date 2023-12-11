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

    let mut node_mappings: HashMap<String, (String, String)> = HashMap::new();
    for line in input.lines().skip(2) {
        let node_regex = regex!(r"(\w{3}) = \((\w{3}), (\w{3})\)");
        let captures = node_regex.captures(line).unwrap();
        let key = captures[1].to_string();
        node_mappings.insert(
            key.clone(),
            (captures[2].to_string(), captures[3].to_string()),
        );
    }

    let mut num_steps: usize = 0;
    let mut current_node_key = &String::from("AAA");
    for direction in directions.iter().cycle() {
        if current_node_key == "ZZZ" {
            break;
        }
        num_steps += 1;

        let (left_node_key, right_node_key) = &node_mappings[current_node_key];
        current_node_key = match direction {
            Direction::Left => left_node_key,
            Direction::Right => right_node_key,
        };
    }

    println!("Task 1: {num_steps}");

    Ok(())
}
