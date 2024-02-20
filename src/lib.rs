use std::fs::read_to_string;

pub fn print_result<T>(day: u8, challenge: u8, resolver: fn(&T) -> isize, data: &T) {
    println!("Result for day {day} challenge {challenge}: {}", resolver(data));
}

pub fn read_lines(filename: &str) -> Vec<String> {
    to_lines(&read_to_string(filename).unwrap())
}

pub fn to_lines(content: &str) -> Vec<String> {
    content.lines().map(String::from).collect()
}
