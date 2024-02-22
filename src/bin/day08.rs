use core::slice::Iter;
use std::collections::HashMap;

use adventofcode_rust::{print_result, read_lines};

fn get_map<'a>(lines: &mut Iter<'a, String>) -> HashMap<&'a str, (&'a str, &'a str)> {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let (key, values) = line.split_once(" = ").unwrap();
        let (left, right) = values.split_once(", ").unwrap();
        map.insert(key, (left.trim_start_matches("("), right.trim_end_matches(")")));
    }
    map
}

fn challenge1(lines: &Vec<String>) -> isize {
    let mut lines_iter = lines.iter();
    let instructions: Vec<char> = lines_iter.next().unwrap().chars().collect();
    lines_iter.next(); // waste empty line

    let map = get_map(&mut lines_iter);

    let mut counter = 0;
    let mut current_line = "AAA";
    let mut instructions_index: usize = 0;
    while current_line != "ZZZ" {
        if instructions[instructions_index] == 'R' {
            current_line = &map[current_line].1;
        } else {
            current_line = &map[current_line].0;
        }
        counter += 1;
        instructions_index += 1;
        instructions_index = instructions_index % instructions.len();
    }
    counter
}

fn challenge2(_lines: &Vec<String>) -> isize {
    0
}

fn main() {
    let input = read_lines("data/2023/day08.txt");
    print_result(8, 1, challenge1, &input);
    // print_result(8, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_lines;

    const EXAMPLE_INPUT1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_INPUT2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT1)), 2);
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT2)), 6);
    }

    #[test]
    #[ignore]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT1)), 71503);
    }
}
