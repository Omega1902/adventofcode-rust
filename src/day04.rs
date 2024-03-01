use crate::util::{extract_pos_numbers, print_full_result, read_lines};

fn get_my_winning_number_count(scratchcard: &str) -> usize {
    let (winning_numbers, selected_numbers) = scratchcard.split_once(" | ").unwrap();
    let winners = extract_pos_numbers(winning_numbers);
    let selecteds = extract_pos_numbers(selected_numbers);
    selecteds.iter().filter(|selected| winners.contains(selected)).count()
}

fn challenge1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(get_my_winning_number_count)
        .map(|my_winning_numbers| if my_winning_numbers == 0 { 0 } else { 2usize.pow((my_winning_numbers - 1) as u32) })
        .sum()
}

fn challenge2(lines: &Vec<String>) -> usize {
    let mut instances = vec![1usize; lines.len() + 1];
    lines
        .iter()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(card_label, card_content)| {
            (extract_pos_numbers(card_label)[0], get_my_winning_number_count(card_content))
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

pub fn main() {
    let filename = "data/2023/day04.txt";
    print_full_result(4, filename, read_lines, challenge1, challenge2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_lines;

    const EXAMPLE_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 13);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 30);
    }
}
