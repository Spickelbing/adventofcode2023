use eyre::Result;
use history::History;
use std::fs;

mod history;

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    let histories: Vec<History> = input
        .lines()
        .map(numbers_in)
        .map(History::from_iter)
        .collect();

    let solution_task_1: i32 = histories
        .iter()
        .map(History::extrapolate_right)
        .flat_map(|history| history.last())
        .sum();

    let solution_task_2: i32 = histories
        .iter()
        .map(History::extrapolate_left)
        .flat_map(|history| history.first())
        .sum();

    println!("Task 1: {solution_task_1}");
    println!("Task 2: {solution_task_2}");

    Ok(())
}

fn numbers_in(string: &str) -> Vec<i32> {
    string.split_whitespace().flat_map(str::parse).collect()
}
