use crate::util::{extract_pos_numbers, print_full_result, read_lines};
use std::iter::zip;

fn calc_winning_sum(time: usize, distance_threshold: usize) -> usize {
    let is_even = time % 2 == 0;
    let calculate_to = if is_even { time / 2 } else { (time - 1) / 2 };
    for i in 1..=calculate_to {
        let distance = (time - i) * i;
        if distance > distance_threshold {
            let score: usize = (calculate_to - i + 1) * 2;
            if is_even {
                return score - 1;
            } else {
                return score;
            }
        }
    }
    0
}

fn challenge1(lines: &Vec<String>) -> usize {
    let times = extract_pos_numbers(&lines[0]);
    let distances_threshold = extract_pos_numbers(&lines[1]);
    zip(times, distances_threshold)
        .map(|(time, distance_threshold)| calc_winning_sum(time, distance_threshold))
        .product()
}

fn challenge2(lines: &Vec<String>) -> usize {
    let times = extract_pos_numbers(&lines[0].replace(' ', ""));
    let distances_threshold = extract_pos_numbers(&lines[1].replace(' ', ""));
    zip(times, distances_threshold)
        .map(|(time, distance_threshold)| calc_winning_sum(time, distance_threshold))
        .product()
}

pub fn main() {
    let filename = "data/2023/day06.txt";
    print_full_result(6, filename, read_lines, challenge1, challenge2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_lines;

    const EXAMPLE_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 288);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 71503);
    }
}
