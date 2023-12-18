use camel_cards::Hand;
use eyre::{Error, Result};
use std::fs;

mod camel_cards;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?;

    let mut hands_with_bids: Vec<(Hand, usize)> = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        hands_with_bids.push((hand.parse()?, bid.parse()?));
    }
    hands_with_bids.sort_by(|(hand_a, _), (hand_b, _)| hand_b.cmp(hand_a));

    let total_winnings = hands_with_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (index, (_, bid))| acc + (index + 1) * bid);

    println!("Task 1: {total_winnings}");

    Ok(())
}
