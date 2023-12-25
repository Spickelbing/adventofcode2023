use eyre::Result;
use grid::Grid;
use std::fs;

mod grid;

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let grid = Grid::try_from(input.as_str())?;
    println!("Task 1: {}", solve_task_1(&grid));
    Ok(())
}

fn solve_task_1(grid: &Grid) -> usize {
    let mut current_position = grid.start_position();
    let mut maybe_previous_position = None;

    let mut loop_length = 0;

    loop {
        let &next_position = grid
            .positions_of_traversable_pipes_from(current_position)
            .iter()
            .filter(|coordinate| match maybe_previous_position {
                Some(previous_position) => previous_position != **coordinate,
                _ => true,
            })
            .next()
            .unwrap();

        loop_length += 1;

        if next_position == grid.start_position() {
            break;
        } else {
            maybe_previous_position = Some(current_position);
            current_position = next_position;
        }
    }

    loop_length / 2
}
