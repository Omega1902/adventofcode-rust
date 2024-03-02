use crate::util::{print_full_result, read_lines};
use {once_cell::sync::Lazy, regex::Regex};

fn game_is_possible(runs: &str, red: &u32, green: &u32, blue: &u32) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap());
    runs.split(';').all(|run| {
        RE.captures_iter(run).all(|cap| {
            if &cap["color"] == "red" {
                return cap["count"].parse::<u32>().unwrap() <= *red;
            }
            if &cap["color"] == "green" {
                return cap["count"].parse::<u32>().unwrap() <= *green;
            }
            cap["count"].parse::<u32>().unwrap() <= *blue
        })
    })
}

fn challenge1(lines: &Vec<String>) -> usize {
    let red = 12u32;
    let green = 13u32;
    let blue = 14u32;
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split(": ");
            let game = parts.next().unwrap().split_at(5).1;
            if game_is_possible(parts.next().unwrap(), &red, &green, &blue) {
                return game.parse().unwrap();
            }
            0
        })
        .sum()
}

fn get_game_power(runs: &str) -> usize {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap());
    //TODO: split into a single run, extract how many items of each color where neded, calculate the minimum of each color of all runs of the game, multiply them.
    let (red, green, blue) = runs
        .split(';')
        .map(|run| {
            RE.captures_iter(run)
                .map(|cap| {
                    if &cap["color"] == "red" {
                        return (cap["count"].parse::<usize>().unwrap(), 0, 0);
                    }
                    if &cap["color"] == "green" {
                        return (0, cap["count"].parse::<usize>().unwrap(), 0);
                    }
                    (0, 0, cap["count"].parse::<usize>().unwrap())
                })
                .fold((0, 0, 0), |cur, next| {
                    (
                        if cur.0 > next.0 { cur.0 } else { next.0 },
                        if cur.1 > next.1 { cur.1 } else { next.1 },
                        if cur.2 > next.2 { cur.2 } else { next.2 },
                    )
                })
        })
        .fold((0, 0, 0), |cur, next| {
            (
                if cur.0 > next.0 { cur.0 } else { next.0 },
                if cur.1 > next.1 { cur.1 } else { next.1 },
                if cur.2 > next.2 { cur.2 } else { next.2 },
            )
        });
    red * green * blue
}

fn challenge2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split(": ");
            parts.next(); // waste it
            get_game_power(parts.next().unwrap())
        })
        .sum()
}

pub fn main() {
    let filename = "data/2023/day02.txt";
    print_full_result(2, filename, read_lines, challenge1, challenge2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_lines;

    const EXAMPLE_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_lines(EXAMPLE_INPUT)), 8);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2(&to_lines(EXAMPLE_INPUT)), 2286);
    }
}
