use adventofcode_rust::extract_numbers;
use adventofcode_rust::print_result;
use adventofcode_rust::read_lines;
use std::collections::HashMap;

fn parse_input(lines: &Vec<String>) -> (Vec<usize>, HashMap<&str, Vec<Vec<usize>>>) {
    let mut seeds: Vec<usize> = vec![];
    let mut map: HashMap<&str, Vec<Vec<usize>>> = HashMap::new();
    let blocks: Vec<&[String]> = lines.split(|line| line == &"").collect();

    for block_lines in blocks {
        if block_lines[0].starts_with("seeds: ") {
            for line in block_lines {
                seeds.append(&mut extract_numbers(line))
            }
        } else {
            let name = block_lines[0].split_once(" ").unwrap().0;
            map.insert(
                name,
                block_lines
                    .iter()
                    .map(|line| extract_numbers(line))
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

fn challenge1(lines: &Vec<String>) -> isize {
    let (mut seeds, map) = parse_input(lines);
    // calc location from seeds
    let mut find: &str = "seed";
    while find != "location" {
        let current_key = map.keys().filter(|key| key.starts_with(find)).collect::<Vec<_>>()[0];
        let current_map = map.get(current_key).unwrap();
        seeds = seeds
            .iter()
            .map(|old_index| lookup_index(*old_index, current_map))
            .collect();
        find = current_key.split_once("-to-").unwrap().1;
    }
    *seeds.iter().min().unwrap() as isize
}

fn challenge2(_lines: &Vec<String>) -> isize {
    0
}

fn main() {
    let input = read_lines("data/2023/day5_input.txt");
    print_result(5, 1, challenge1, &input);
    print_result(5, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_lines;

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
        assert_eq!(
            seeds
                .iter()
                .map(|seed| lookup_index(*seed, &map))
                .collect::<Vec<usize>>(),
            soil
        )
    }

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 35);
    }

    #[test]
    #[ignore = "Not implemented yet"]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 30);
    }
}
