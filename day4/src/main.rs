use eyre::{Error, Result};
use std::{collections::HashSet, fs};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?;

    let numbers: Vec<(HashSet<u32>, HashSet<u32>)> = input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning, have) = numbers.split_once('|').unwrap();
            let parse_numbers = |string: &str| -> HashSet<u32> {
                string
                    .split_whitespace()
                    .map(str::parse::<u32>)
                    .map(Result::unwrap)
                    .collect()
            };
            (parse_numbers(winning), parse_numbers(have))
        })
        .collect();

    let n_matches_per_card: Vec<usize> = numbers
        .iter()
        .map(|(winning, have)| winning.intersection(have).count())
        .collect();

    // task 1
    let points = n_matches_per_card.iter().map(|&n_matches| match n_matches {
        0 => 0,
        _ => 1 << (n_matches - 1),
    });
    println!("Task 1: {}", points.sum::<usize>());

    // task 2
    let mut n_scratchcards = 0;
    let mut card_indices_to_process: Vec<usize> = (0..numbers.len()).collect();
    let mut new_card_indices = Vec::<usize>::new();

    while !card_indices_to_process.is_empty() {
        for card_index in card_indices_to_process {
            n_scratchcards += 1;
            let card_matches = n_matches_per_card[card_index];
            let indices_of_won_cards = ((card_index + 1)..(card_index + 1 + card_matches))
                .take_while(|&card_index| card_index < numbers.len());
            new_card_indices.extend(indices_of_won_cards);
        }
        card_indices_to_process = new_card_indices;
        new_card_indices = Vec::new();
    }

    println!("Task 2: {}", n_scratchcards);

    Ok(())
}
