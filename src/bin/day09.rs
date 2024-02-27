use adventofcode_rust::{extract_numbers, pairwise, print_result, read_lines};

fn get_missing_number(row: &Vec<i64>, history: bool) -> i64 {
    if row.iter().all(|item| item == &0) {
        return 0;
    }
    let new_row: Vec<i64> = pairwise(row).map(|(left, right)| right - left).collect();

    if history {
        row.first().unwrap() - get_missing_number(&new_row, history)
    } else {
        row.last().unwrap() + get_missing_number(&new_row, history)
    }
}

fn challenge1(lines: &Vec<String>) -> isize {
    lines
        .iter()
        .map(|line| extract_numbers(&line))
        .map(|row| get_missing_number(&(row.iter().map(|&item| item as i64).collect()), false))
        .sum::<i64>() as isize
}

fn challenge2(lines: &Vec<String>) -> isize {
    lines
        .iter()
        .map(|line| extract_numbers(&line))
        .map(|row| get_missing_number(&(row.iter().map(|&item| item as i64).collect()), true))
        .sum::<i64>() as isize
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
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 2);
    }
}
