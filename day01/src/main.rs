use std::{fs, io};

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
    static WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digits = Vec::new();

    for search_offset in 0..string.len() {
        let substring = &string[search_offset..];

        let first_char = substring.chars().next().unwrap();
        if let Some(number) = first_char.to_digit(10) {
            digits.push(number);
        } else {
            for (number, &word) in WORDS.iter().enumerate() {
                if substring.starts_with(word) {
                    digits.push((number + 1) as u32);
                }
            }
        }
    }

    digits
}
