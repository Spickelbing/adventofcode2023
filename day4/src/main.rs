use std::{fs, collections::HashSet};
use eyre::{Result, Error};

fn main() -> Result<(), Error> {
  let input = fs::read_to_string("input")?;

  let numbers:Vec<(HashSet<u32>, HashSet<u32>)> = input.lines().map(|line| {
    let (_, numbers) = line.split_once(':').unwrap();
    let (winning, have) = numbers.split_once('|').unwrap();
    let parse_numbers = |string: &str| -> HashSet<u32> {string.split_whitespace().map(str::parse::<u32>).map(Result::unwrap).collect()};
    (parse_numbers(winning), parse_numbers(have))
  }).collect();

  // task 1
  let points = numbers.iter().map(|(winning, have)| {
    let intersections = winning.intersection(have).count();
    if intersections == 0 { 0 } else { 1 << (intersections - 1) }
  });

  println!("Task 1: {}", points.sum::<usize>());

  Ok(())
}
