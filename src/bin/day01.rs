use adventofcode_rust::print_result;
use adventofcode_rust::read_lines;
use regex::Regex;
use std::collections::HashMap;

fn challenge1(lines: &Vec<String>) -> i32 {
    let regex = Regex::new(r"\d").unwrap();

    return lines
        .iter()
        // find all digit in this line
        .map(|line| {
            regex
                .find_iter(line)
                .map(|finding| finding.as_str())
                .collect::<Vec<&str>>()
        })
        // maps list of digits into a 2 digits string
        .map(|findings| format!("{}{}", findings[0], findings.last().unwrap()))
        // convert from string to number
        .map(|number_str| number_str.parse::<i32>().unwrap())
        .sum();
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
    return map[num];
}

fn challenge2(lines: &Vec<String>) -> i32 {
    let regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    return lines
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
        .map(|line| {
            regex
                .find_iter(&line)
                .map(|finding| finding.as_str().to_owned())
                .collect()
        })
        // maps list of digits into a 2 digits string
        .map(|findings: Vec<String>| format!("{}{}", to_digit(&findings[0]), to_digit(findings.last().unwrap())))
        // convert from string to number
        .map(|number_str| number_str.parse::<i32>().unwrap())
        .sum();
}

fn main() {
    let challenge1_ex = read_lines("data/2023/day1_example_input.txt");
    let challenge = read_lines("data/2023/day1_input.txt");
    print_result(1, 1, challenge1, &challenge1_ex, &challenge, 142);
    let challenge2_ex = read_lines("data/2023/day1_example_input2.txt");
    print_result(1, 2, challenge2, &challenge2_ex, &challenge, 281);
}
