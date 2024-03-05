use crate::util::{extract_pos_numbers, print_full_result, read_lines};
use std::collections::HashMap;
use std::usize;

fn parse_input(lines: &[String]) -> (Vec<usize>, HashMap<&str, Vec<Vec<usize>>>) {
    let mut seeds: Vec<usize> = vec![];
    let mut map: HashMap<&str, Vec<Vec<usize>>> = HashMap::new();
    let blocks: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();

    for block_lines in blocks {
        if block_lines[0].starts_with("seeds: ") {
            for line in block_lines {
                seeds.append(&mut extract_pos_numbers(line))
            }
        } else {
            let name = block_lines[0].split_once(' ').unwrap().0;
            map.insert(
                name,
                block_lines
                    .iter()
                    .map(|line| extract_pos_numbers(line))
                    .filter(|numbers| !numbers.is_empty())
                    .collect(),
            );
        }
    }
    (seeds, map)
}

fn lookup_index(old_index: usize, map: &Vec<Vec<usize>>) -> usize {
    for map_item in map {
        if map_item[1] <= old_index && map_item[1] + map_item[2] > old_index {
            return old_index + map_item[0] - map_item[1];
        }
    }
    old_index
}

fn add_numbers(operand1: usize, operand2: isize) -> usize {
    //TODO: find a better solution than this helper function!
    (operand1 as isize + operand2) as usize
}

type Ranges = Vec<(usize, usize)>;
fn lookup_index_range_item(ranges: Ranges, start: usize, end: usize, offset: isize) -> (Ranges, Ranges) {
    let mut remaining_ranges: Ranges = vec![];
    let mut transformed_ranges: Ranges = vec![];
    for range in ranges {
        if start <= range.0 && end > range.0 {
            let new_start = add_numbers(range.0, offset);
            let new_end: usize;
            if end > range.1 {
                // range fits completely in
                new_end = add_numbers(range.1, offset);
            } else {
                // first part of the range fits in
                new_end = add_numbers(end - 1, offset);
                remaining_ranges.push((end, range.1));
            }
            transformed_ranges.push((new_start, new_end));
        } else if start <= range.1 && end >= range.1 {
            // last part of the range fits in
            let new_end = add_numbers(range.1, offset);
            let new_start = add_numbers(start, offset);
            remaining_ranges.push((range.0, start - 1));
            transformed_ranges.push((new_start, new_end));
        } else {
            // range does not fit in at all
            remaining_ranges.push(range);
        }
    }
    (transformed_ranges, remaining_ranges)
}

fn lookup_index_range(old_range: (usize, usize), map: &[Vec<usize>]) -> Ranges {
    let mut remaining_ranges = vec![old_range];
    let mut result: Ranges = vec![];
    for map_item in map {
        let (mut cur_transformed, cur_remaining) = lookup_index_range_item(
            remaining_ranges,
            map_item[1],
            map_item[1] + map_item[2],
            map_item[0] as isize - map_item[1] as isize,
        );
        result.append(&mut cur_transformed);
        remaining_ranges = cur_remaining;
    }
    result.append(&mut remaining_ranges);
    result
}

fn calc_location_from_seeds(seeds: Vec<usize>, map: &HashMap<&str, Vec<Vec<usize>>>) -> Vec<usize> {
    let mut indexes = seeds;
    let mut find: &str = "seed";
    while find != "location" {
        let current_key = map.keys().filter(|key| key.starts_with(find)).collect::<Vec<_>>()[0];
        let current_map = map.get(current_key).unwrap();
        indexes = indexes.iter().map(|old_index| lookup_index(*old_index, current_map)).collect();
        find = current_key.split_once("-to-").unwrap().1;
    }
    indexes
}

fn calc_location_from_seed_ranges(seed_ranges: Ranges, map: &HashMap<&str, Vec<Vec<usize>>>) -> Ranges {
    let mut indexes = seed_ranges;
    let mut find: &str = "seed";
    while find != "location" {
        let current_key = map.keys().filter(|key| key.starts_with(find)).collect::<Vec<_>>()[0];
        let current_map = map.get(current_key).unwrap();
        let mut new_indexes: Ranges = vec![];
        indexes
            .iter()
            .map(|old_index| lookup_index_range(*old_index, current_map))
            .for_each(|mut new_indexes_part| new_indexes.append(&mut new_indexes_part));
        indexes = new_indexes;
        find = current_key.split_once("-to-").unwrap().1;
    }
    indexes
}

fn challenge1(lines: &[String]) -> usize {
    let (seeds, map) = parse_input(lines);
    *calc_location_from_seeds(seeds, &map).iter().min().unwrap()
}

fn challenge2(lines: &[String]) -> usize {
    let (seeds, map) = parse_input(lines);
    let seeds_transformed: Ranges = seeds.chunks(2).map(|range| (range[0], range[0] + range[1] - 1)).collect();
    calc_location_from_seed_ranges(seeds_transformed, &map).iter().map(|range| range.0).min().unwrap()
}

pub fn main() {
    let filename = "data/2023/day05.txt";
    print_full_result(5, filename, read_lines, challenge1, challenge2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_lines;

    const EXAMPLE_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_transform() {
        let seeds: Vec<usize> = vec![79, 14, 55, 13];
        let map: Vec<Vec<usize>> = vec![vec![50, 98, 2], vec![52, 50, 48]];
        let soil: Vec<usize> = vec![81, 14, 57, 13];
        assert_eq!(seeds.iter().map(|seed| lookup_index(*seed, &map)).collect::<Vec<usize>>(), soil)
    }

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 35);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 46);
    }
}
