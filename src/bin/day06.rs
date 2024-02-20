use adventofcode_rust::{print_result, read_lines};

fn challenge1(lines: &Vec<String>) -> isize {
    0
}

fn challenge2(lines: &Vec<String>) -> isize {
    0
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
    #[ignore]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 467835);
    }
}
