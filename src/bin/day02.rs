use adventofcode_rust::print_result;
use adventofcode_rust::read_lines;
use {once_cell::sync::Lazy, regex::Regex};

fn game_is_possible(runs: &str, red: &u32, green: &u32, blue: &u32) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap());
    runs.split(";").all(|run| {
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

fn challenge1(lines: &Vec<String>) -> i32 {
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

fn get_game_power(runs: &str) -> i32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap());
    //TODO: split into a single run, extract how many items of each color where neded, calculate the minimum of each color of all runs of the game, multiply them.
    let (red, green, blue) = runs
        .split(";")
        .map(|run| {
            RE.captures_iter(run)
                .map(|cap| {
                    if &cap["color"] == "red" {
                        return (cap["count"].parse::<i32>().unwrap(), 0, 0);
                    }
                    if &cap["color"] == "green" {
                        return (0, cap["count"].parse::<i32>().unwrap(), 0);
                    }
                    (0, 0, cap["count"].parse::<i32>().unwrap())
                })
                .fold((0, 0, 0), |cur: (i32, i32, i32), next: (i32, i32, i32)| {
                    (
                        if cur.0 > next.0 { cur.0 } else { next.0 },
                        if cur.1 > next.1 { cur.1 } else { next.1 },
                        if cur.2 > next.2 { cur.2 } else { next.2 },
                    )
                })
        })
        .fold((0, 0, 0), |cur: (i32, i32, i32), next: (i32, i32, i32)| {
            (
                if cur.0 > next.0 { cur.0 } else { next.0 },
                if cur.1 > next.1 { cur.1 } else { next.1 },
                if cur.2 > next.2 { cur.2 } else { next.2 },
            )
        });
    red * green * blue
}

fn challenge2(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split(": ");
            parts.next(); // waste it
            get_game_power(parts.next().unwrap())
        })
        .sum()
}

fn main() {
    let challenge_input_example = read_lines("data/2023/day2_example_input.txt");
    let challenge_input = read_lines("data/2023/day2_input.txt");
    print_result(2, 1, challenge1, &challenge_input_example, &challenge_input, 8);
    print_result(2, 2, challenge2, &challenge_input_example, &challenge_input, 2286);
}
