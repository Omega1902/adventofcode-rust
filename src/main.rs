use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn challenge1(lines: &Vec<String>) -> i32 {
    let regex = Regex::new(r"\d").unwrap();

    return lines.iter()
        // find all digit in this line
        .map(|line| regex.find_iter(line).map(|finding| finding.as_str()).collect::<Vec<&str>>())
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

    return lines.iter()
        // split connected words
        .map(|line| line.replace("oneight", "oneeight").replace("threeight", "threeeight").replace("nineight", "nineeight").replace("twone", "twoone").replace("sevenine", "sevennine").replace("eightwo", "eighttwo").replace("eighthree", "eightthree"))
        // find all digits in this line
        .map(|line| regex.find_iter(&line).map(|finding| finding.as_str().to_owned()).collect())
        // maps list of digits into a 2 digits string
        .map(|findings: Vec<String>| format!("{}{}", to_digit(&findings[0]), to_digit(findings.last().unwrap())))
        // convert from string to number
        .map(|number_str| number_str.parse::<i32>().unwrap())
        .sum();
}

fn main() {
    let day1challenge1_ex = read_lines("data/2023/day1_example_input.txt");
    let day1challenge1 = read_lines("data/2023/day1_input.txt");
    let day1challenge1_ex_res: i32 = challenge1(&day1challenge1_ex);
    assert_eq!(day1challenge1_ex_res, 142);
    println!("Day1 challenge 1 seems to work");
    println!("Result challenge 1: {}", challenge1(&day1challenge1));
    let day1challenge2_ex = read_lines("data/2023/day1_example_input2.txt");
    let day1challenge2_ex_res: i32 = challenge2(&day1challenge2_ex);
    assert_eq!(day1challenge2_ex_res, 281);
    println!("Day1 challenge 2 seems to work");
    println!("Result challenge 2: {}", challenge2(&day1challenge1));
    // Wrong answers: 54095 (to low)
}
