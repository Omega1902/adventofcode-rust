use std::fs::read_to_string;

pub fn print_result<T>(day: &u8, challenge: &u8, resolver: fn(&T) -> i32, data_test: &T, data: &T, result: &i32) {
    println!("Testing day {day} challenge {challenge}...");
    assert_eq!(resolver(data_test), *result);
    println!(
        "Seems to work fine. Result for day {day} challenge {challenge}: {}",
        resolver(data)
    );
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
