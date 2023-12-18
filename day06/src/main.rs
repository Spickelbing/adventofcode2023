use eyre::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    let compute_n_ways_to_win_per_race = |times_and_distances: &Vec<(usize, usize)>| -> Vec<usize> {
        times_and_distances
            .iter()
            .map(|&(time, distance)| {
                let mut n_ways_to_win = 0;

                for time_held_down_button in 1..time {
                    let distance_traveled = (time - time_held_down_button) * time_held_down_button;
                    if distance_traveled > distance {
                        n_ways_to_win += 1;
                    }
                }

                n_ways_to_win
            })
            .collect()
    };

    // task 1
    {
        let times_and_distances: Vec<(usize, usize)> = {
            let mut numbers_per_line = input.lines().map(numbers_in);
            let times = numbers_per_line.next().unwrap();
            let distances = numbers_per_line.next().unwrap();
            times.into_iter().zip(distances).collect()
        };

        let n_ways_to_win_per_race = compute_n_ways_to_win_per_race(&times_and_distances);
        println!(
            "Task 1: {}",
            n_ways_to_win_per_race.iter().product::<usize>()
        );
    }

    // task 2
    {
        let merged_numbers: Vec<_> = input
            .lines()
            .map(|line| {
                let (_, numbers) = line.split_once(':').unwrap();
                numbers.replace(' ', "").parse().unwrap()
            })
            .collect();
        let [time, distance]: [usize; 2] = merged_numbers.try_into().unwrap();

        let n_ways_to_win_task_2 = compute_n_ways_to_win_per_race(&vec![(time, distance)]);
        println!("Task 2: {}", n_ways_to_win_task_2.iter().product::<usize>());
    }

    Ok(())
}

fn numbers_in(string: &str) -> Vec<usize> {
    string
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect()
}
