use std::fs;

use eyre::{anyhow, Result};

enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl TryFrom<char> for Tile {
    type Error = eyre::Error;
    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            '|' => Ok(Tile::Pipe(Pipe::Vertical)),
            '-' => Ok(Tile::Pipe(Pipe::Horizontal)),
            'L' => Ok(Tile::Pipe(Pipe::NorthEast)),
            'J' => Ok(Tile::Pipe(Pipe::NorthWest)),
            'F' => Ok(Tile::Pipe(Pipe::SouthEast)),
            '7' => Ok(Tile::Pipe(Pipe::SouthWest)),
            _ => Err(anyhow!("invalid character")),
        }
    }
}

impl Tile {
    fn is_start(&self) -> bool {
        match self {
            Tile::Start => true,
            _ => false,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(Tile::try_from)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, tile)| tile.is_start())
                .map(|(x, _)| (x, y))
        })
        .unwrap();

    println!("{start_pos:?}");

    Ok(())
}
