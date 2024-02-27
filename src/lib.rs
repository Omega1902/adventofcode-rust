use std::fs::read_to_string;
use {once_cell::sync::Lazy, regex::Regex};

pub fn pairwise<I>(iter: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: IntoIterator + Clone,
{
    // pairwise([1,2,3,4,5]) -> (1,2) (2,3) (3,4) (4,5)
    let mut right = iter.clone().into_iter();
    right.next();
    iter.into_iter().zip(right)
}

pub fn extract_numbers(number_str: &str) -> Vec<isize> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"-?\d+").expect("Could not parse integers regex"));
    RE.find_iter(number_str).map(|finding| finding.as_str().parse().expect("Could not parse as integer")).collect()
}

pub fn extract_pos_numbers(number_str: &str) -> Vec<usize> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").expect("Could not parse positive integers regex"));
    RE.find_iter(number_str).map(|finding| finding.as_str().parse().expect("Could not parse as integer")).collect()
}

pub fn print_result<T: ?Sized, I: std::fmt::Display>(day: u8, challenge: u8, resolver: fn(&T) -> I, data: &T) {
    println!("Result for day {day} challenge {challenge}: {}", resolver(data));
}

pub fn read_lines(filename: &str) -> Vec<String> {
    to_lines(&read_to_string(filename).expect("File does not exist"))
}

pub fn to_lines(content: &str) -> Vec<String> {
    content.lines().map(String::from).collect()
}

pub fn read_chars(filename: &str) -> Vec<Vec<char>> {
    to_chars(&read_to_string(filename).expect("File dows not exist"))
}

pub fn to_chars(content: &str) -> Vec<Vec<char>> {
    content.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairwise() {
        let pairwise_vec: Vec<(i8, i8)> = pairwise([1i8, 2i8, 3i8, 4i8, 5i8]).collect();
        assert_eq!(pairwise_vec, vec![(1i8, 2i8), (2i8, 3i8), (3i8, 4i8), (4i8, 5i8)]);
        let pairwise_vec: Vec<(i8, i8)> = pairwise([1i8]).collect();
        assert_eq!(pairwise_vec, vec![]);
        let pairwise_vec: Vec<(i8, i8)> = pairwise([]).collect();
        assert_eq!(pairwise_vec, vec![]);
        let pairwise_vec: Vec<(char, char)> = pairwise(['A', 'B', 'C', 'D', 'E']).collect();
        assert_eq!(pairwise_vec, vec![('A', 'B'), ('B', 'C'), ('C', 'D'), ('D', 'E')]);
    }

    const EXAMPLE_INPUT1: &str = "0 3 9 12 145";
    const EXAMPLE_INPUT2: &str = "0 3 9 12 -145";

    #[test]
    fn test_extract_pos_numbers() {
        assert_eq!(extract_pos_numbers(EXAMPLE_INPUT1), vec![0, 3, 9, 12, 145]);
        assert_eq!(extract_pos_numbers(EXAMPLE_INPUT2), vec![0, 3, 9, 12, 145]);
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers(EXAMPLE_INPUT1), vec![0, 3, 9, 12, 145]);
        assert_eq!(extract_numbers(EXAMPLE_INPUT2), vec![0, 3, 9, 12, -145]);
    }
}
