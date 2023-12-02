use eyre::anyhow;
use lazy_regex::regex;
use std::{collections::HashMap, fs};

#[derive(Eq, PartialEq, Hash, Debug)]
enum Cube {
    Red,
    Green,
    Blue,
}

type CubeHandful = HashMap<Cube, u32>;

#[derive(Debug)]
struct GameRecord {
    id: u32,
    revelations: Vec<CubeHandful>,
}

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("input")?;

    let mut game_records = Vec::new();
    for line in input.lines() {
        let game_record = GameRecord::try_from(line)?;
        game_records.push(game_record);
    }

    println!("Task 1: {}", solve_task_1(&game_records));

    Ok(())
}

fn solve_task_1(game_records: &Vec<GameRecord>) -> u32 {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    game_records
        .iter()
        .filter(|game_record| {
            game_record.revelations.iter().all(|handful| {
                let red_impossible = handful
                    .get(&Cube::Red)
                    .is_some_and(|&red_amount| red_amount > red_max);
                let green_impossible = handful
                    .get(&Cube::Green)
                    .is_some_and(|&green_amount| green_amount > green_max);
                let blue_impossible = handful
                    .get(&Cube::Blue)
                    .is_some_and(|&blue_amount| blue_amount > blue_max);

                !red_impossible && !green_impossible && !blue_impossible
            })
        })
        .map(|game_record| game_record.id)
        .sum()
}

impl TryFrom<&str> for GameRecord {
    type Error = eyre::Error;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let id_regex = regex!(r"Game\s+(?<id>[1-9][0-9]*):\s+");
        let id_captures = id_regex.captures(line).ok_or(anyhow!("invalid game id"))?;
        let id = id_captures["id"].parse::<u32>().unwrap(); // ok because regex guarantees valid u32

        let parse_cube_amount = |string: &str| -> Result<(Cube, u32), Self::Error> {
            let captures = regex!(r"(?<amount>[1-9][0-9]*)\s+(?<cube>\w+)")
                .captures(string)
                .ok_or(anyhow!("invalid cube amount"))?;

            let amount = captures["amount"].parse::<u32>().unwrap(); // ok because regex guarantees valid u32
            let cube = match &captures["cube"] {
                "blue" => Cube::Blue,
                "green" => Cube::Green,
                "red" => Cube::Red,
                _ => return Err(anyhow!("invalid type of cube")),
            };

            Ok((cube, amount))
        };

        let revelations_start = id_captures[0].len();

        let mut revelations = Vec::new();
        for handful in line[revelations_start..]
            .split_terminator(";")
            .map(str::trim_start)
        {
            let mut revelation = HashMap::new();
            for cube_amount in handful.split_terminator(",") {
                let (cube, amount) = parse_cube_amount(cube_amount)?;
                revelation.insert(cube, amount);
            }
            revelations.push(revelation);
        }

        Ok(GameRecord {
            id: id,
            revelations: revelations,
        })
    }
}
