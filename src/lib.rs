use std::fs::read_to_string;
use {once_cell::sync::Lazy, regex::Regex};

pub fn extract_numbers(number_str: &str) -> Vec<usize> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    RE.find_iter(number_str)
        .map(|finding| finding.as_str().parse().unwrap())
        .collect()
}

pub fn print_result<T: ?Sized>(day: u8, challenge: u8, resolver: fn(&T) -> isize, data: &T) {
    println!("Result for day {day} challenge {challenge}: {}", resolver(data));
}

pub fn read_lines(filename: &str) -> Vec<String> {
    to_lines(&read_to_string(filename).unwrap())
}

pub fn to_lines(content: &str) -> Vec<String> {
    content.lines().map(String::from).collect()
}
