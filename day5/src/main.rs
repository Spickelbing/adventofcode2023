use eyre::{Error, Result};
use std::fs;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?.replace("\r\n", "\n");
    let (first_line, remaining_input) = input.split_once("\n\n").unwrap();

    let (_, seed_numbers) = first_line.split_once(':').unwrap();
    let seed_numbers = numbers_in(seed_numbers);

    let mut maps = Vec::new();
    let paragraphs = remaining_input.split_terminator("\n\n");

    for paragraph in paragraphs {
        let mut map = Vec::new();

        for line in paragraph.lines() {
            let numbers = numbers_in(line);
            if let [x, y, z] = numbers[..] {
                map.push((x, y, z));
            }
        }

        maps.push(map);
    }

    let seed_locations: Vec<usize> = seed_numbers
        .iter()
        .map(|&seed_number| {
            let mut number_to_map = seed_number;
            for map in &maps {
                for &(dest_start, source_start, range) in map {
                    if (source_start..(source_start + range)).contains(&number_to_map) {
                        let offset = source_start.abs_diff(number_to_map);
                        number_to_map = dest_start + offset;
                        break;
                    }
                }
            }
            number_to_map
        })
        .collect();

    let minimum_location = seed_locations.iter().min().unwrap();
    println!("Task 1: {minimum_location}");

    Ok(())
}

fn numbers_in(string: &str) -> Vec<usize> {
    string
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect()
}
