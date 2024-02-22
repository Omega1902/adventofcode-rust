use core::slice::Iter;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use adventofcode_rust::{print_result, read_lines};

fn get_map(lines: &mut Iter<'_, String>) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let (key, values) = line.split_once(" = ").unwrap();
        let (left, right) = values.split_once(", ").unwrap();
        map.insert(
            key.to_owned(),
            (
                left.trim_start_matches("(").to_owned(),
                right.trim_end_matches(")").to_owned(),
            ),
        );
    }
    map
}

fn walk_map<'a>(map: &'a HashMap<String, (String, String)>, start: &str, instruction: char) -> &'a str {
    if instruction == 'L' {
        &map[start].0
    } else {
        &map[start].1
    }
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
        current_line = walk_map(&map, current_line, instructions[instructions_index]);
        counter += 1;
        instructions_index += 1;
        instructions_index = instructions_index % instructions.len();
    }
    counter
}

fn challenge2(lines: &Vec<String>) -> isize {
    let mut lines_iter = lines.iter();
    let instructions: Vec<char> = lines_iter.next().unwrap().chars().collect();
    lines_iter.next(); // waste empty line

    let map = get_map(&mut lines_iter);

    let mut counter = 0;
    let mut current_lines: HashSet<&str> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| node.as_str())
        .collect();
    let mut instructions_index: usize = 0;
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{spinner} {human_pos}").unwrap());
    spinner.enable_steady_tick(Duration::from_millis(200));
    while !current_lines.iter().all(|node| node.ends_with("Z")) {
        spinner.set_position(counter);
        current_lines = current_lines
            .iter()
            .map(|current_line| walk_map(&map, current_line, instructions[instructions_index]))
            .collect();
        counter += 1;
        instructions_index += 1;
        instructions_index = instructions_index % instructions.len();
    }
    spinner.finish_with_message("Done");
    counter as isize
}

fn main() {
    let input = read_lines("data/2023/day08.txt");
    print_result(8, 1, challenge1, &input);
    print_result(8, 2, challenge2, &input);
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

    const EXAMPLE_INPUT3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT1)), 2);
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT2)), 6);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT3)), 6);
    }
}
