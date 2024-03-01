use crate::util::{print_full_result, read_lines};
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

fn get_rank_joker(card: &char) -> u8 {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("{card} is not a card"),
    }
}

fn get_hand_rank(cards: &str) -> u8 {
    let mut map: HashMap<char, u8> = HashMap::new();
    for card in cards.chars() {
        map.insert(card, *map.get(&card).unwrap_or(&0) + 1);
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

fn get_hand_rank_joker(cards: &str) -> u8 {
    let mut map: HashMap<char, u8> = HashMap::new();
    for card in cards.chars() {
        map.insert(card, *map.get(&card).unwrap_or(&0) + 1);
    }

    let jokers = map.remove(&'J').unwrap_or(0);

    let values: Vec<&u8> = map.values().collect();
    match values.iter().count() {
        5 => 1, // jokers would already have been reduced
        4 => 2, // covers 1 pair and high-card + 1 joker
        3 => {
            if values.contains(&&2u8) {
                3 + jokers // covers 2 pairs and 1 pair + 1 joker
            } else {
                4 // covers high-card + 2 jokers and 3 of a kind
            }
        }
        2 => {
            if values.contains(&&2u8) && !values.contains(&&1u8) {
                5 // coveres 2 pair + 1 joker and 1 pair + 3 of a kind
            } else {
                6 // covers 1 single card + 1,2,3 or 4 of a kind + 3,2,1 or 0 jokers
            }
        }
        1 => 7,
        0 => 7, // covers only jokers
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

fn compare_hands_joker(hand1: &String, hand2: &String) -> Ordering {
    let cards1 = hand1.split_at(5).0;
    let cards2 = hand2.split_at(5).0;
    let cards1_rank = get_hand_rank_joker(cards1);
    let cards2_rank = get_hand_rank_joker(cards2);
    if cards1_rank > cards2_rank {
        return Ordering::Greater;
    }
    if cards1_rank < cards2_rank {
        return Ordering::Less;
    }
    for (card1, card2) in zip(cards1.chars(), cards2.chars()) {
        let card1_rank = get_rank_joker(&card1);
        let card2_rank = get_rank_joker(&card2);
        if card1_rank > card2_rank {
            return Ordering::Greater;
        }
        if card1_rank < card2_rank {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn challenge1(lines: &Vec<String>) -> usize {
    let mut sortable_lines = lines.clone();
    sortable_lines.sort_by(compare_hands);
    sortable_lines
        .iter()
        .enumerate()
        .map(|(i, line)| line.split_at(6).1.parse::<usize>().unwrap() * (i + 1))
        .sum::<usize>()
}

fn challenge2(lines: &Vec<String>) -> usize {
    let mut sortable_lines = lines.clone();
    sortable_lines.sort_by(compare_hands_joker);
    sortable_lines
        .iter()
        .enumerate()
        .map(|(i, line)| line.split_at(6).1.parse::<usize>().unwrap() * (i + 1))
        .sum::<usize>()
}

pub fn main() {
    let filename = "data/2023/day07.txt";
    print_full_result(7, filename, read_lines, challenge1, challenge2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_lines;

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
