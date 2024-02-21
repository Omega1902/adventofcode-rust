use adventofcode_rust::{extract_numbers, print_result, read_lines};
use std::iter::zip;

fn calc_winning_sum(time: usize, distance_threshold: usize) -> isize {
    let is_even = time % 2 == 0;
    let calculate_to = if is_even { time / 2 } else { (time - 1) / 2 };
    for i in 1..=calculate_to {
        let distance = (time - i) * i;
        if distance > distance_threshold {
            let score: isize = (calculate_to - i + 1) as isize * 2;
            if is_even {
                return score - 1;
            } else {
                return score;
            }
        }
    }
    0
}

fn challenge1(lines: &Vec<String>) -> isize {
    let times = extract_numbers(&lines[0]);
    let distances_threshold = extract_numbers(&lines[1]);
    zip(times, distances_threshold)
        .map(|(time, distance_threshold)| calc_winning_sum(time, distance_threshold))
        .product()
}

fn challenge2(lines: &Vec<String>) -> isize {
    let times = extract_numbers(&lines[0].replace(" ", ""));
    let distances_threshold = extract_numbers(&lines[1].replace(" ", ""));
    zip(times, distances_threshold)
        .map(|(time, distance_threshold)| calc_winning_sum(time, distance_threshold))
        .product()
}

fn main() {
    let input = read_lines("data/2023/day06.txt");
    print_result(6, 1, challenge1, &input);
    print_result(6, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_lines;

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
