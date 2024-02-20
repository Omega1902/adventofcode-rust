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

fn challenge1(lines: &Vec<String>) -> isize {
    let mut sum: isize = 0;
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
                    .parse::<isize>()
                    .unwrap();
            }
            index = word_start + length;
        }
    }
    sum
}

fn is_number(index: usize, line: &str) -> bool {
    NUMBERS.contains(&(line.get(index..index + 1).unwrap_or(IGNORE_CHAR)))
}

fn get_numbers_on_index(index: usize, line: Option<&String>) -> Vec<usize> {
    if line.is_none() {
        return vec![];
    }

    let mut result: Vec<usize> = vec![];
    let temp_line = line.unwrap();
    if index > 0 {
        if is_number(index - 1, temp_line) {
            result.push(index - 1);
        }
    }
    if is_number(index, temp_line) {
        result.push(index)
    }
    if is_number(index + 1, temp_line) {
        result.push(index + 1);
    }
    result
}

fn get_number_count(row_findings: &Vec<usize>) -> usize {
    if row_findings.is_empty() {
        0
    } else if row_findings.len() == 1 || row_findings.len() >= 3 || row_findings[1] - row_findings[0] == 1 {
        1
    } else {
        2
    }
}

fn get_full_number(index: usize, line: &str) -> isize {
    let mut end = index + 1; // non-inclusive
    while let Some(c) = line.get(end..end + 1) {
        if !NUMBERS.contains(&c) {
            break;
        }
        end += 1;
    }
    let mut start = index; //inclusive
    while let Some(c) = line.get(start - 1..start) {
        if !NUMBERS.contains(&c) {
            break;
        }
        start -= 1;
        if start == 0 {
            break;
        }
    }
    line.get(start..end).unwrap().parse().unwrap()
}

fn get_full_numbers_of_vec(numbers: &Vec<usize>, line: Option<&String>) -> isize {
    if numbers.is_empty() {
        return 1;
    }
    if numbers.len() == 1 || numbers.len() >= 3 || numbers[1] - numbers[0] == 1 {
        get_full_number(numbers[0], line.unwrap())
    } else {
        get_full_number(numbers[0], line.unwrap()) * get_full_number(numbers[1], line.unwrap())
    }
}

fn get_gear_value(index: usize, line: &str, prev_line: Option<&String>, next_line: Option<&String>) -> isize {
    let top: Vec<usize> = get_numbers_on_index(index, prev_line);
    let mut left: Option<usize> = None;
    let mut right: Option<usize> = None;
    let bot: Vec<usize> = get_numbers_on_index(index, next_line);
    if index > 0 && is_number(index - 1, line) {
        left = Some(index - 1);
    }
    if is_number(index + 1, line) {
        right = Some(index + 1);
    }

    // validate that there are exactly 2 numbers surrounding gear
    let mut numbers_count = get_number_count(&top) + get_number_count(&bot);
    if left.is_some() {
        numbers_count += 1;
    }
    if right.is_some() {
        numbers_count += 1;
    }
    if numbers_count != 2 {
        return 0;
    }

    // find the complete surrounding numbers
    let mut product: isize = 1;
    if left.is_some() {
        product *= get_full_number(left.unwrap(), line);
    }
    if right.is_some() {
        product *= get_full_number(right.unwrap(), line);
    }
    product *= get_full_numbers_of_vec(&top, prev_line);
    product *= get_full_numbers_of_vec(&bot, next_line);

    product
}

fn challenge2(lines: &Vec<String>) -> isize {
    // sum the product of all gear numbers
    let mut sum: isize = 0;
    for (line_number, line) in lines.iter().enumerate() {
        let prev_line = if line_number == 0 {
            None
        } else {
            lines.get(line_number - 1)
        };
        let next_line = lines.get(line_number + 1);
        let mut index: usize = 0;
        let mut remaining_line = line.get(index..).unwrap();
        while let Some(gear) = remaining_line.find("*") {
            index += gear;
            sum += get_gear_value(index, line, prev_line, next_line);
            index += 1;
            remaining_line = line.get(index..).unwrap();
        }
    }
    sum
}

fn main() {
    let input = read_lines("data/2023/day3_input.txt");
    print_result(3, 1, challenge1, &input);
    print_result(3, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_lines;

    const EXAMPLE_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 4361);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 467835);
    }
}
