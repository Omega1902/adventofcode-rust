use adventofcode_rust::print_result;
use adventofcode_rust::read_lines;
use {once_cell::sync::Lazy, regex::Regex};

fn extract_numbers(number_str: &str) -> Vec<usize> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    RE.find_iter(number_str)
        .map(|finding| finding.as_str().parse().unwrap())
        .collect()
}

fn get_my_winning_number_count(scratchcard: &str) -> usize {
    let (winning_numbers, selected_numbers) = scratchcard.split_once(" | ").unwrap();
    let winners = extract_numbers(winning_numbers);
    let selecteds = extract_numbers(selected_numbers);
    selecteds.iter().filter(|selected| winners.contains(selected)).count()
}

fn challenge1(lines: &Vec<String>) -> isize {
    lines
        .iter()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(get_my_winning_number_count)
        .map(|my_winning_numbers| {
            if my_winning_numbers == 0 {
                0
            } else {
                2isize.pow((my_winning_numbers - 1) as u32)
            }
        })
        .sum()
}

fn challenge2(lines: &Vec<String>) -> isize {
    let mut instances = vec![1isize; lines.len() + 1];
    lines
        .iter()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(card_label, card_content)| {
            (
                extract_numbers(card_label)[0],
                get_my_winning_number_count(card_content),
            )
        })
        .map(|(card_index, my_winning_cards)| {
            let own_value = instances[card_index];
            if my_winning_cards > 0 {
                for i in card_index + 1..=card_index + my_winning_cards {
                    instances[i] += own_value;
                }
            }
            own_value
        })
        .sum()
}

fn main() {
    let input_example = read_lines("data/2023/day4_example_input.txt");
    let input = read_lines("data/2023/day4_input.txt");
    print_result(4, 1, challenge1, &input_example, &input, 13);
    print_result(4, 2, challenge2, &input_example, &input, 30);
}
