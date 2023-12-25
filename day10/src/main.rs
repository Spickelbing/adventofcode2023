use eyre::Result;
use grid::{Coordinate, Grid};
use std::{collections::HashSet, fs};

mod grid;

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    let grid = Grid::try_from(input.as_str())?;

    let (start_x, start_y) = grid
        .iter_rows()
        .enumerate()
        .find_map(|(y, row)| {
            row.enumerate()
                .find(|(_, tile)| tile.is_start())
                .map(|(x, _)| (x, y))
        })
        .unwrap();

    let mut visited_coordinates = HashSet::new();
    let maybe_shortest_loop_length = find_length_of_shortest_loop(
        &grid,
        Coordinate::new(start_x, start_y),
        None,
        &mut visited_coordinates,
    );
    println!("Task 1: {maybe_shortest_loop_length:?}",);

    Ok(())
}

fn find_length_of_shortest_loop(
    grid: &Grid,
    start_position: Coordinate,
    previous_position: Option<Coordinate>,
    visited_coordinates: &mut HashSet<Coordinate>,
) -> Option<usize> {
    let mut maybe_loop_length = None;

    let pipe_positions =
        grid.positions_of_traversable_pipes_from(start_position.x, start_position.y);
    println!(
        "traversable pipes from ({}, {}): {:?}",
        start_position.x, start_position.y, pipe_positions
    );

    for coordinate_choice in pipe_positions {
        println!("chose ({}, {})", coordinate_choice.x, coordinate_choice.y);
        if previous_position.is_some_and(|it| it == coordinate_choice)
            || visited_coordinates.contains(&coordinate_choice)
        {
            continue;
        } else if grid.get(coordinate_choice)?.is_start() {
            return Some(1);
        }

        visited_coordinates.insert(coordinate_choice);

        let maybe_choice_loop_length = find_length_of_shortest_loop(
            grid,
            coordinate_choice,
            Some(start_position),
            visited_coordinates,
        )
        .map(|length| length + 1);

        visited_coordinates.remove(&coordinate_choice);

        if let Some(choice_loop_length) = maybe_choice_loop_length {
            if maybe_loop_length.is_none()
                || maybe_loop_length.is_some_and(|loop_length| loop_length > choice_loop_length)
            {
                maybe_loop_length = maybe_choice_loop_length;
            }
        }
    }

    maybe_loop_length
}
