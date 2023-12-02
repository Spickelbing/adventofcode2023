use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;

    let digits_task_1: Vec<_> = input.lines().map(digits_in).collect();

    println!(
        "Part 1: {}",
        calibration_values(digits_task_1).iter().sum::<u32>()
    );

    let digits_task_2: Vec<_> = input
        .lines()
        .map(digits_and_spelled_out_digits_in)
        .collect();

    println!(
        "Part 2: {}",
        calibration_values(digits_task_2).iter().sum::<u32>()
    );

    return Ok(());
}

fn calibration_values(digits: Vec<Vec<u32>>) -> Vec<u32> {
    digits
        .iter()
        .map(|digits| {
            let first = digits.first().unwrap_or(&0);
            let last = digits.last().unwrap_or(&0);
            first * 10 + last
        })
        .collect()
}

fn digits_in(string: &str) -> Vec<u32> {
    string
        .chars()
        .filter_map(|char| -> Option<u32> { char.to_digit(10) })
        .collect()
}

fn digits_and_spelled_out_digits_in(string: &str) -> Vec<u32> {
    static NUMBERS_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"((?<word>one|two|three|four|five|six|seven|eight|nine)|(?<digit>[0-9]))")
            .unwrap()
    });
    static WORD_TO_NUMBER: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
        HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ])
    });

    let mut digits = Vec::new();
    for search_offset in 0..string.len() {
        if let Some(captures) = NUMBERS_REGEX.captures_at(string, search_offset) {
            if let Some(word_match) = captures.name("word") {
                let word = word_match.as_str();
                let number = WORD_TO_NUMBER.get(word).unwrap(); // regex guarantees success
                digits.push(number);
            }

            if let Some(digit_match) = captures.name("digit") {
                let number = &digit_match.as_str().parse::<u32>().unwrap(); // regex guarantees success
                digits.push(number);
            }
        }
    }

    digits
}
