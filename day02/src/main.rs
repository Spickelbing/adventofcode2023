use eyre::anyhow;
use lazy_regex::regex;
use std::{cmp::max, fs};

#[derive(Debug)]
struct CubeHandful {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

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
    println!("Task 2: {}", solve_task_2(&game_records));

    Ok(())
}

fn solve_task_2(game_records: &Vec<GameRecord>) -> u32 {
    let powers = game_records.iter().map(|game_record| {
        let mut blue_min = 0;
        let mut green_min = 0;
        let mut red_min = 0;

        for revelation in &game_record.revelations {
            blue_min = max(blue_min, revelation.blue_cubes);
            red_min = max(red_min, revelation.red_cubes);
            green_min = max(green_min, revelation.green_cubes);
        }

        blue_min * green_min * red_min
    });

    powers.sum()
}

fn solve_task_1(game_records: &Vec<GameRecord>) -> u32 {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    game_records
        .iter()
        .filter(|game_record| {
            game_record.revelations.iter().all(|handful| {
                handful.red_cubes <= red_max
                    && handful.green_cubes <= green_max
                    && handful.blue_cubes <= blue_max
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

        let revelations_start = id_captures[0].len();

        let mut revelations = Vec::new();
        for handful in line[revelations_start..]
            .split_terminator(";")
            .map(str::trim_start)
        {
            let mut revelation = CubeHandful {
                red_cubes: 0,
                green_cubes: 0,
                blue_cubes: 0,
            };
            for cube_amount_string in handful.split_terminator(",") {
                let captures = regex!(r"(?<amount>[1-9][0-9]*)\s+(?<cube>\w+)")
                    .captures(cube_amount_string)
                    .ok_or(anyhow!("invalid cube amount"))?;
                let amount = captures["amount"].parse::<u32>().unwrap(); // ok because regex guarantees valid u32
                
                match &captures["cube"] {
                    "blue" => revelation.blue_cubes = amount,
                    "green" => revelation.green_cubes = amount,
                    "red" => revelation.red_cubes = amount,
                    _ => return Err(anyhow!("invalid type of cube")),
                }
            }
            revelations.push(revelation);
        }

        Ok(GameRecord {
            id: id,
            revelations: revelations,
        })
    }
}
