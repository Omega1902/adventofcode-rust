use adventofcode_rust::{print_result, read_lines};

fn get_rank(card: &char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("{card} is not a card"),
    }
}

fn challenge1(_lines: &Vec<String>) -> isize {
    0
}

fn challenge2(_lines: &Vec<String>) -> isize {
    0
}

fn main() {
    let input = read_lines("data/2023/day07.txt");
    print_result(7, 1, challenge1, &input);
    print_result(7, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_lines;

    const EXAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 6440);
    }

    #[test]
    #[ignore]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 71503);
    }
}
