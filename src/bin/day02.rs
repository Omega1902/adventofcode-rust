use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use adventofcode_rust::read_lines;
use adventofcode_rust::print_result;

fn game_is_possible(runs: &str, red: &u32, green: &u32, blue: &u32) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap());
    return runs.split(";").all(|run| RE.captures_iter(run).all(|cap| {
        if &cap["color"] == "red" {
            return cap["count"].parse::<u32>().unwrap() <= *red;
        }
        if &cap["color"] == "green" {
            return cap["count"].parse::<u32>().unwrap() <= *green;
        }
        return cap["count"].parse::<u32>().unwrap() <= *blue;
    }));
}

fn challenge1(lines: &Vec<String>) -> i32 {
    let red = 12u32;
    let green = 13u32;
    let blue = 14u32;
    return lines.iter().map(|line| {
        let mut parts = line.split(": ");
        let game = parts.next().unwrap().split_at(5).1;
        if game_is_possible(parts.next().unwrap(), &red, &green, &blue) {
            return game.parse().unwrap();
        }
        return 0;
    }).sum();
}

fn get_game_power(runs: &str) -> i32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap());
    //TODO: split into a single run, extract how many items of each color where neded, calculate the minimum of each color of all runs of the game, multiply them.
    // runs.split(";").map(|run| RE.captures_iter(run).)
    //.reduce(f)
    return 0;
}

fn challenge2(lines: &Vec<String>) -> i32 {
    return lines.iter().map(|line|{        
        let mut parts = line.split(": ");
        parts.next(); // waste it
        return get_game_power(parts.next().unwrap());
    }).sum();
}

fn main() {
    let challenge_input_example = read_lines("data/2023/day2_example_input.txt");
    let challenge_input = read_lines("data/2023/day2_input.txt");
    print_result(&2u8, &1u8, challenge1, &challenge_input_example, &challenge_input, &8i32);
    print_result(&2u8, &2u8, challenge2, &challenge_input_example, &challenge_input, &2286i32);
}
