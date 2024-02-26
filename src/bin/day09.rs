use adventofcode_rust::{extract_numbers, pairwise, print_result, read_lines};

fn get_missing_number(row: &Vec<i64>) -> i64 {
    if row.iter().all(|item| item == &0) {
        return 0;
    }
    let new_row: Vec<i64> = pairwise(row).map(|(left, right)| right - left).collect();

    row.last().unwrap() + get_missing_number(&new_row)
}

fn challenge1(lines: &Vec<String>) -> isize {
    lines
        .iter()
        .map(|line| extract_numbers(&line))
        .map(|row| get_missing_number(&(row.iter().map(|&item| item as i64).collect())))
        .sum::<i64>() as isize
}

fn challenge2(_lines: &Vec<String>) -> isize {
    0
}

fn main() {
    let input = read_lines("data/2023/day09.txt");
    print_result(9, 1, challenge1, &input);
    print_result(9, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_lines;

    const EXAMPLE_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 114);
    }

    #[test]
    #[ignore]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 6);
    }
}
