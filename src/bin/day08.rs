use core::slice::Iter;
use hashbrown::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
use num::Integer;
use std::time::Duration;

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

fn find_final_node(
    start_node: &str,
    end_node_known: bool,
    map: &HashMap<String, (String, String)>,
    instructions: &Vec<char>,
) -> usize {
    let mut counter = 0;
    let mut current_node = start_node;
    let end_node_found = if end_node_known {
        |current_node: &str| current_node == "ZZZ"
    } else {
        |current_node: &str| current_node.ends_with("Z")
    };
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{spinner} {human_pos}").unwrap());
    spinner.enable_steady_tick(Duration::from_millis(200));
    while !end_node_found(current_node) {
        spinner.set_position(counter as u64);
        current_node = walk_map(&map, current_node, instructions[counter % instructions.len()]);
        counter += 1;
    }
    counter
}

fn challenge1(lines: &Vec<String>) -> isize {
    let mut lines_iter = lines.iter();
    let instructions: Vec<char> = lines_iter.next().unwrap().chars().collect();
    lines_iter.next(); // waste empty line

    let map = get_map(&mut lines_iter);

    find_final_node("AAA", true, &map, &instructions) as isize
}

fn challenge2(lines: &Vec<String>) -> isize {
    // this implementation assumes that each individual path from **A to **Z is repeated endlessly
    // therefore it calulates the length of all paths on its own, and then calculates the Lowest Common Multiple
    let mut lines_iter = lines.iter();
    let instructions: Vec<char> = lines_iter.next().unwrap().chars().collect();
    lines_iter.next(); // waste empty line

    let map = get_map(&mut lines_iter);

    map.keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| node.as_str())
        .map(|start_node| find_final_node(start_node, false, &map, &instructions))
        .reduce(|cur, next| cur.lcm(&next))
        .unwrap() as isize
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
