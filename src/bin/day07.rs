use adventofcode_rust::{print_result, read_lines};
use std::{cmp::Ordering, collections::HashMap, iter::zip, usize};

fn get_rank(card: &char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("{card} is not a card"),
    }
}

fn get_hand_rank(cards: &str) -> u8 {
    let mut map: HashMap<char, u8> = HashMap::new();
    for card in cards.chars() {
        if map.contains_key(&card) {
            map.insert(card, map[&card] + 1);
        } else {
            map.insert(card, 1);
        }
    }
    let values: Vec<&u8> = map.values().collect();
    match values.iter().count() {
        5 => 1,
        4 => 2,
        3 => {
            if values.contains(&&2u8) {
                3
            } else {
                4
            }
        }
        2 => {
            if values.contains(&&2u8) {
                5
            } else {
                6
            }
        }
        1 => 7,
        i => panic!("wrong amount of cards: {i} - {cards}"),
    }
}

fn compare_hands(hand1: &String, hand2: &String) -> Ordering {
    let cards1 = hand1.split_at(5).0;
    let cards2 = hand2.split_at(5).0;
    let cards1_rank = get_hand_rank(cards1);
    let cards2_rank = get_hand_rank(cards2);
    if cards1_rank > cards2_rank {
        return Ordering::Greater;
    }
    if cards1_rank < cards2_rank {
        return Ordering::Less;
    }
    for (card1, card2) in zip(cards1.chars(), cards2.chars()) {
        let card1_rank = get_rank(&card1);
        let card2_rank = get_rank(&card2);
        if card1_rank > card2_rank {
            return Ordering::Greater;
        }
        if card1_rank < card2_rank {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn challenge1(lines: &Vec<String>) -> isize {
    let mut sortable_lines = lines.clone();
    sortable_lines.sort_by(compare_hands);
    sortable_lines
        .iter()
        .enumerate()
        .map(|(i, line)| line.split_at(6).1.parse::<usize>().unwrap() * (i + 1))
        .sum::<usize>() as isize
}

fn challenge2(_lines: &Vec<String>) -> isize {
    0
}

fn main() {
    let input = read_lines("data/2023/day07.txt");
    print_result(7, 1, challenge1, &input);
    print_result(7, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_lines;

    const EXAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 6440);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 5905);
    }
}
