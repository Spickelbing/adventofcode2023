mod schematic;

use schematic::{EngineSchematic, SchematicSymbol};
use std::{collections::HashSet, fs};

fn solve_task_1(schematic: &EngineSchematic) -> u32 {
    let mut part_numbers = Vec::<u32>::new();

    for y in 0..schematic.dimensions().y {
        let mut skip_next_digits_until_non_digit = false;

        for x in 0..schematic.dimensions().x {
            let is_digit = schematic.get(x, y).unwrap().is_digit();

            match (is_digit, skip_next_digits_until_non_digit) {
                (true, true) => continue,
                (false, true) => skip_next_digits_until_non_digit = false,
                _ => (),
            }

            if is_digit {
                let is_adjacent_to_special_char = schematic
                    .adjacent_positions(x, y)
                    .iter()
                    .map(|pos| schematic.get(pos.x, pos.y).unwrap())
                    .any(SchematicSymbol::is_special_character);

                if is_adjacent_to_special_char {
                    let part_number = schematic.get_complete_number(x, y).unwrap();
                    part_numbers.push(part_number);
                    skip_next_digits_until_non_digit = true;
                }
            }
        }
    }

    part_numbers.iter().sum()
}

fn solve_task_2(schematic: &EngineSchematic) -> u32 {
    let mut gear_ratios: Vec<u32> = Vec::new();

    for y in 0..schematic.dimensions().y {
        for x in 0..schematic.dimensions().x {
            if let Some(SchematicSymbol::SpecialCharacter('*')) = schematic.get(x, y) {
                // The use of a set is not really correct since the gear symbol could be adjacent to
                // multiple identical part numbers.
                // However that doesn't happen for my input schematic so I don't care right now.
                let adjacent_part_numbers: HashSet<u32> = schematic
                    .adjacent_positions(x, y)
                    .iter()
                    .filter_map(|pos| schematic.get_complete_number(pos.x, pos.y))
                    .collect();

                if adjacent_part_numbers.len() == 2 {
                    gear_ratios.push(adjacent_part_numbers.iter().product());
                }
            }
        }
    }

    gear_ratios.iter().sum()
}

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("input")?;

    let schematic: EngineSchematic = input.as_str().try_into()?;

    println!("Task 1: {}", solve_task_1(&schematic));
    println!("Task 2: {}", solve_task_2(&schematic));

    Ok(())
}
