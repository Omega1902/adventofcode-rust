use crate::util::{print_full_result, read_lines};
use regex::Regex;
use std::collections::HashMap;

fn challenge1(lines: &[String]) -> usize {
    let regex = Regex::new(r"\d").unwrap();

    lines
        .iter()
        // find all digit in this line
        .map(|line| regex.find_iter(line).map(|finding| finding.as_str()).collect::<Vec<&str>>())
        // maps list of digits into a 2 digits string
        .map(|findings| format!("{}{}", findings[0], findings.last().unwrap()))
        // convert from string to number
        .map(|number_str| number_str.parse::<usize>().unwrap())
        .sum()
}

fn to_digit(num: &str) -> &str {
    if num.len() == 1 {
        return num;
    }
    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    map[num]
}

fn challenge2(lines: &[String]) -> usize {
    let regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    lines
        .iter()
        // split connected words
        .map(|line| {
            line.replace("oneight", "oneeight")
                .replace("threeight", "threeeight")
                .replace("nineight", "nineeight")
                .replace("twone", "twoone")
                .replace("sevenine", "sevennine")
                .replace("eightwo", "eighttwo")
                .replace("eighthree", "eightthree")
        })
        // find all digits in this line
        .map(|line| regex.find_iter(&line).map(|finding| finding.as_str().to_owned()).collect())
        // maps list of digits into a 2 digits string
        .map(|findings: Vec<String>| format!("{}{}", to_digit(&findings[0]), to_digit(findings.last().unwrap())))
        // convert from string to number
        .map(|number_str| number_str.parse::<usize>().unwrap())
        .sum()
}

pub fn main() {
    let filename = "data/2023/day01.txt";
    print_full_result(1, filename, read_lines, challenge1, challenge2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_lines;

    const EXAMPLE_INPUT1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const EXAMPLE_INPUT2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT1)), 142);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT2)), 281);
    }
}
