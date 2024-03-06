use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::util::{extract_pos_numbers, print_full_result, read_lines};

fn find_first_block(group: usize, parts: &[char]) -> Option<usize> {
    let mut start_index = 0;
    let mut length = 0;
    let mut tail = parts;
    let mut must_have = false;
    while length < group {
        let (head, new_tail) = tail.split_first()?;
        tail = new_tail;
        match *head {
            '.' => {
                if must_have {
                    return None;
                }
                start_index = parts.len() - tail.len();
                length = 0;
            }
            '#' => {
                length += 1;
                must_have = true;
            }
            '?' => length += 1,
            _ => panic!("character not allowed"),
        }
    }
    Some(start_index)
}

fn remove_group(group: usize, min_size: usize, parts: &[char]) -> Vec<&[char]> {
    let mut result = vec![];
    let mut i = 0;
    while i <= parts.len() - min_size {
        if parts[i] == '.' {
            i += 1;
            continue;
        }
        let index = match find_first_block(group, &parts[i..]) {
            Some(int) => int,
            None => return result,
        };
        // i in the next loop i should omit until the current index, because thats the first available block
        i += index;
        let must_use = parts[i] == '#';
        // if the following char is '#', then we cannot end our block here
        if parts.get(i + group).unwrap_or(&'.') == &'#' {
            if must_use {
                return result;
            }
            i += 1;
            continue;
        }

        // check if the end is finished now
        if i + group + 1 > parts.len() {
            // no more groups, just add one
            result.push(&parts[0..0]);
            return result;
        }
        result.push(&parts[i + group + 1..]);
        // the first item is '#', so we must use this result
        if must_use {
            return result;
        }
        i += 1;
    }

    result
}

fn _get_possible_varants(parts: &[char], groups: &[usize]) -> usize {
    if groups.is_empty() {
        if parts.contains(&'#') {
            return 0;
        }
        return 1;
    }
    let min_size = groups.iter().sum::<usize>() + groups.len() - 1;
    if parts.len() < min_size {
        return 0;
    }
    let (group, next_groups) = groups.split_first().unwrap();

    let all_blocks = remove_group(*group, min_size, parts);

    all_blocks.iter().map(|part| _get_possible_varants(part, next_groups)).sum()
}

fn get_possible_variants(line: &str, unfold: bool) -> usize {
    let (parts_string, groups_string) = line.split_once(' ').expect("file input not correct");
    let groups: Vec<usize> = extract_pos_numbers(groups_string);
    let parts = parts_string.chars().collect_vec();

    if unfold {
        let unfolded_groups = groups.repeat(5);
        let unfolded_parts = [parts_string; 5].join("?").chars().collect_vec();
        _get_possible_varants(&unfolded_parts, &unfolded_groups)
    } else {
        _get_possible_varants(&parts, &groups)
    }
}

fn challenge1(grid: &[String]) -> usize {
    //7653 is right!
    grid.iter().map(|line| get_possible_variants(line, false)).sum()
}

fn challenge2(grid: &[String]) -> usize {
    grid.iter().progress_count(grid.len() as u64).map(|line| get_possible_variants(line, true)).sum()
}

pub fn main() {
    let filename = "data/2023/day12.txt";
    print_full_result(12, filename, read_lines, challenge1, challenge2);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_lines;

    const EXAMPLE_INPUT1: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_find_first_block() {
        assert_eq!(find_first_block(1, &['.', '#']), Some(1));
        assert_eq!(find_first_block(2, &['.', '?', '#']), Some(1));
        assert_eq!(find_first_block(3, &['.', '?', '?', '.', '?', '?', '?']), Some(4));
        assert_eq!(find_first_block(3, &['#', '?', '#']), Some(0));
        assert_eq!(find_first_block(3, &['.', '?', '#', '.']), None);
    }

    #[test]
    fn test_get_possible_variants() {
        assert_eq!(get_possible_variants("#??????#??. 2,7", false), 1);
        assert_eq!(get_possible_variants("#???????#??. 2,7", false), 2);
        assert_eq!(get_possible_variants("#??????#??###. 2,10", false), 1);
        assert_eq!(get_possible_variants("?#?? 2", false), 2);
        assert_eq!(get_possible_variants("??#?#?###????.????. 11,2", false), 9);
        assert_eq!(get_possible_variants("?????.#.?????##??? 2,1,1,2,1,5", false), 3);
        assert_eq!(get_possible_variants("?.????#??.??????#?.# 1,1,5,1,4,1", false), 5);
        assert_eq!(get_possible_variants("??.?#?#?.?..???#.?? 2,4,1,1,1", false), 16);
        assert_eq!(get_possible_variants("?#?###?.#?#????# 1,4,1,1,1", false), 1);

        let lines = to_lines(EXAMPLE_INPUT1);
        assert_eq!(get_possible_variants(&lines[0], false), 1);
        assert_eq!(get_possible_variants(&lines[1], false), 4);
        assert_eq!(get_possible_variants(&lines[2], false), 1);
        assert_eq!(get_possible_variants(&lines[3], false), 1);
        assert_eq!(get_possible_variants(&lines[4], false), 4);
        assert_eq!(get_possible_variants(&lines[5], false), 10);
    }

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT1)), 21);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT1)), 525152);
    }
}
