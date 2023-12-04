use schematic::{EngineSchematic, Pos2, SchematicSymbol};
use std::{collections::HashSet, fs};

mod schematic {
    use eyre::anyhow;
    use std::iter::repeat;

    pub struct EngineSchematic {
        dimensions: Dim2,
        contents: Vec2<SchematicSymbol>,
    }

    impl EngineSchematic {
        fn new(dimensions: Dim2) -> EngineSchematic {
            let mut contents = Vec2::new();

            for _ in 0..dimensions.y {
                let row = Vec::from_iter(repeat(SchematicSymbol::Nothing).take(dimensions.x));
                contents.push(row);
            }

            EngineSchematic {
                dimensions,
                contents,
            }
        }

        pub fn get(&self, x: usize, y: usize) -> Option<SchematicSymbol> {
            self.contents.get(y)?.get(x).cloned()
        }

        pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut SchematicSymbol> {
            self.contents.get_mut(y)?.get_mut(x)
        }

        pub fn dimensions(&self) -> Dim2 {
            self.dimensions.clone()
        }

        pub fn get_complete_number(&self, x: usize, y: usize) -> Option<u32> {
            if let Some(SchematicSymbol::Digit(_)) = self.get(x, y) {
                let mut first_digit_x = x;
                for x_to_check in (0..x).rev() {
                    if !self.get(x_to_check, y).unwrap().is_digit() {
                        break;
                    }
                    first_digit_x -= 1;
                }

                let mut complete_number = 0;
                for x_to_get in first_digit_x..self.dimensions.x {
                    if let SchematicSymbol::Digit(digit) = self.get(x_to_get, y).unwrap() {
                        complete_number = complete_number * 10 + digit;
                    } else {
                        break;
                    }
                }

                Some(complete_number)
            } else {
                None
            }
        }

        pub fn adjacent_positions(&self, x: usize, y: usize) -> Vec<Pos2> {
            let mut adjacent_positions = Vec::new();

            let x_not_first = x > 0;
            let x_not_last = x + 1 < self.dimensions.x;
            let y_not_first = y > 0;
            let y_not_last = y + 1 < self.dimensions.y;

            if x_not_first {
                adjacent_positions.push(Pos2 { x: x - 1, y });
            }
            if x_not_last {
                adjacent_positions.push(Pos2 { x: x + 1, y });
            }
            if y_not_first {
                adjacent_positions.push(Pos2 { x, y: y - 1 });
            }
            if y_not_last {
                adjacent_positions.push(Pos2 { x, y: y + 1 });
            }
            if x_not_first && y_not_first {
                adjacent_positions.push(Pos2 { x: x - 1, y: y - 1 });
            }
            if x_not_last && y_not_last {
                adjacent_positions.push(Pos2 { x: x + 1, y: y + 1 });
            }
            if x_not_first && y_not_last {
                adjacent_positions.push(Pos2 { x: x - 1, y: y + 1 });
            }
            if x_not_last && y_not_first {
                adjacent_positions.push(Pos2 { x: x + 1, y: y - 1 });
            }

            adjacent_positions
        }
    }

    impl TryFrom<&str> for EngineSchematic {
        type Error = eyre::Error;

        fn try_from(schematic_string: &str) -> Result<Self, Self::Error> {
            let size_x = schematic_string
                .lines()
                .next()
                .ok_or(anyhow!("input string is empty"))?
                .len();
            let size_y = schematic_string.lines().count();

            let mut schematic = EngineSchematic::new(Dim2 {
                x: size_x,
                y: size_y,
            });

            for (y, line) in schematic_string.lines().enumerate() {
                for (x, char) in line.chars().enumerate() {
                    let to_modify = schematic
                        .get_mut(x, y)
                        .ok_or(anyhow!("line {y} is too long, expected {size_x}"))?;

                    if let Some(digit) = char.to_digit(10) {
                        *to_modify = SchematicSymbol::Digit(digit);
                    } else if "+-*/=#@%$&".contains(char) {
                        *to_modify = SchematicSymbol::SpecialCharacter(char);
                    } else if char != '.' {
                        return Err(anyhow!(
                            "line {y} contains and invalid character in column {x}: {char}"
                        ));
                    }
                }
            }

            Ok(schematic)
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    pub enum SchematicSymbol {
        Digit(u32),
        Nothing,
        SpecialCharacter(char),
    }

    impl SchematicSymbol {
        pub fn is_special_character(self) -> bool {
            match self {
                SchematicSymbol::SpecialCharacter(_) => true,
                _ => false,
            }
        }

        pub fn is_digit(self) -> bool {
            match self {
                SchematicSymbol::Digit(_) => true,
                _ => false,
            }
        }
    }

    #[derive(Clone)]
    pub struct Dim2 {
        pub x: usize,
        pub y: usize,
    }

    #[derive(Clone)]
    pub struct Pos2 {
        pub x: usize,
        pub y: usize,
    }

    type Vec2<T> = Vec<Vec<T>>;
}

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("input")?;

    let schematic: EngineSchematic = input.as_str().try_into()?;
    let dimensions = schematic.dimensions();

    // task 1
    let mut part_numbers = HashSet::<u32>::new();

    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            if schematic.get(x, y).unwrap().is_special_character() {
                let positions_of_adjacent_digits: Vec<Pos2> = schematic
                    .adjacent_positions(x, y)
                    .into_iter()
                    .filter(|position| {
                        schematic
                            .get(position.x, position.y)
                            .is_some_and(SchematicSymbol::is_digit)
                    }).collect();

                let adjacent_part_numbers = positions_of_adjacent_digits.iter().map(|position| {
                    schematic
                        .get_complete_number(position.x, position.y)
                        .unwrap()
                });

                for part_number in adjacent_part_numbers {
                    part_numbers.insert(part_number);
                }
            }
        }
    }

    println!("Task 1: {}", part_numbers.iter().sum::<u32>());

    Ok(())
}
