use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct History {
    values: VecDeque<i32>,
}

impl FromIterator<i32> for History {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        History {
            values: VecDeque::from_iter(iter),
        }
    }
}

impl History {
    fn build_sequence_of_differences(&self) -> Vec<VecDeque<i32>> {
        let mut sequences = vec![self.values.clone()];

        loop {
            let last_sequence = sequences.last().unwrap();

            if last_sequence.iter().all(|&difference| difference == 0) {
                break;
            }

            sequences.push(
                last_sequence
                    .iter()
                    .tuple_windows()
                    .map(|(first, second)| second - first)
                    .collect(),
            );
        }

        sequences
    }

    pub fn extrapolate_right(&self) -> History {
        let mut sequences = self.build_sequence_of_differences();

        sequences.last_mut().unwrap().push_back(0);

        for sequence_index in (0..sequences.len() - 1).rev() {
            let increment = *sequences[sequence_index + 1].back().unwrap();
            let last_value = *sequences[sequence_index].back().unwrap();
            sequences[sequence_index].push_back(last_value + increment);
        }

        sequences.into_iter().next().unwrap().into_iter().collect()
    }

    pub fn extrapolate_left(&self) -> History {
        let mut sequences = self.build_sequence_of_differences();

        sequences.last_mut().unwrap().push_front(0);

        for sequence_index in (0..sequences.len() - 1).rev() {
            let decrement = *sequences[sequence_index + 1].front().unwrap();
            let first_value = *sequences[sequence_index].front().unwrap();
            sequences[sequence_index].push_front(first_value - decrement);
        }

        sequences.into_iter().next().unwrap().into_iter().collect()
    }

    pub fn last(&self) -> Option<i32> {
        self.values.back().copied()
    }

    pub fn first(&self) -> Option<i32> {
        self.values.front().copied()
    }
}
