use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn challenge1(file: &str) -> i32 {
    let lines = read_lines(file);

    let regex = Regex::new(r"\d").unwrap();

    return lines.iter()
        .map(|line| regex.find_iter(line).map(|finding| finding.as_str()).collect::<Vec<&str>>())
        .map(|findings| format!("{}{}", findings[0], findings.last().unwrap()))
        .map(|number_str| number_str.parse::<i32>().unwrap())
        .sum();
}

fn main() {
    let day1challenge1_ex = "data/2023/day1_example_input.txt";
    let day1challenge1 = "data/2023/day1_input.txt";
    let day1challenge1_ex_res: i32 = challenge1(day1challenge1_ex);
    assert_eq!(day1challenge1_ex_res, 142);
    println!("Day1 challenge 1 seems to work");
    println!("Result challenge 1: {}", challenge1(day1challenge1));
}
