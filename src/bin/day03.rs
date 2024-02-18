use adventofcode_rust::print_result;
use adventofcode_rust::read_lines;

const IGNORE_CHAR: &str = ".";
const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn find_next_number(line: &str) -> Option<usize> {
    if line.is_empty() {
        return None;
    }
    let (first_char, remaining_line) = line.split_at(1);
    if NUMBERS.contains(&first_char) {
        return Some(0);
    }
    match find_next_number(remaining_line) {
        Some(i) => return Some(i + 1),
        None => return None,
    }
}

fn check_line_at(index: usize, line: Option<&String>) -> bool {
    // checks whether the line at the specific index might be marked based on the given line
    if line.is_none() {
        return false;
    }
    let prev = line.unwrap().get(index..index + 1).unwrap_or(IGNORE_CHAR);
    prev != IGNORE_CHAR && !NUMBERS.contains(&prev)
}

fn get_number(index: usize, line: &str, previous_line: Option<&String>, next_line: Option<&String>) -> (usize, bool) {
    let current_char = line.get(index..index + 1).unwrap_or(IGNORE_CHAR);
    let mark = check_line_at(index, previous_line) || check_line_at(index, next_line);
    if NUMBERS.contains(&current_char) {
        let (following, already_marked) = get_number(index + 1, line, previous_line, next_line);
        (following + 1, mark || already_marked)
    } else if current_char == IGNORE_CHAR {
        (0, mark)
    } else {
        (0, true)
    }
}

fn challenge1(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for (line_number, line) in lines.iter().enumerate() {
        let mut index: usize = 0;
        let prev_line = if line_number == 0 {
            None
        } else {
            lines.get(line_number - 1)
        };
        let next_line = lines.get(line_number + 1);
        while let Some(next_word) = find_next_number(line.split_at(index).1) {
            let word_start = index + next_word;
            let (length, mark) = get_number(word_start, line, prev_line, next_line);
            let mut previous_marked = false;
            if word_start > 0 {
                previous_marked = check_line_at(word_start - 1, Some(line))
                    || check_line_at(word_start - 1, prev_line)
                    || check_line_at(word_start - 1, next_line);
            }
            if mark || previous_marked {
                sum += line
                    .get(word_start..word_start + length)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
            index = word_start + length;
        }
    }
    sum
}

fn main() {
    let input_example = read_lines("data/2023/day3_example_input.txt");
    let input = read_lines("data/2023/day3_input.txt");
    print_result(3, 1, challenge1, &input_example, &input, 4361);
    // print_result(3, 2, challenge2, &challenge_input_example, &challenge_input, 2286);
}
