use adventofcode_rust::print_result;
use adventofcode_rust::read_lines;
use {once_cell::sync::Lazy, regex::Regex};

fn extract_numbers(number_str: &str) -> Vec<isize> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    RE.find_iter(number_str)
        .map(|finding| finding.as_str().parse().unwrap())
        .collect()
}

fn challenge1(lines: &Vec<String>) -> isize {
    lines
        .iter()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(winning_numbers, selected_numbers)| {
            let winners = extract_numbers(winning_numbers);
            let selecteds = extract_numbers(selected_numbers);
            let my_winning_numbers = selecteds.iter().filter(|selected| winners.contains(selected)).count();
            if my_winning_numbers == 0 {
                0
            } else {
                2isize.pow((my_winning_numbers - 1) as u32)
            }
        })
        .sum()
}

fn main() {
    let input_example = read_lines("data/2023/day4_example_input.txt");
    let input = read_lines("data/2023/day4_input.txt");
    print_result(4, 1, challenge1, &input_example, &input, 13);
    // print_result(4, 2, challenge2, &input_example, &input, 467835);
}
