use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use adventofcode_rust::read_lines;

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

fn challenge1(lines: &Vec<String>, red: &u32, green: &u32, blue: &u32) -> u32 {
    return lines.iter().map(|line| {
        let mut parts = line.split(": ");
        let game = parts.next().unwrap().split_at(5).1;
        if game_is_possible(parts.next().unwrap(), red, green, blue) {
            return game.parse().unwrap();
        }
        return 0;
    }).sum();
}

fn main() {
    let red = 12u32;
    let green = 13u32;
    let blue = 14u32;
    let challenge1_input_example = read_lines("data/2023/day2_example_input.txt");
    let challenge1_input = read_lines("data/2023/day2_input.txt");
    let challenge1_result_example: u32 = challenge1(&challenge1_input_example, &red, &green, &blue);
    assert_eq!(challenge1_result_example, 8);
    println!("Day1 challenge 1 seems to work");
    println!("Result challenge 1: {}", challenge1(&challenge1_input, &red, &green, &blue));
    // let day1challenge2_ex = read_lines("data/2023/day1_example_input2.txt");
    // let day1challenge2_ex_res: i32 = challenge2(&day1challenge2_ex);
    // assert_eq!(day1challenge2_ex_res, 281);
    // println!("Day1 challenge 2 seems to work");
    // println!("Result challenge 2: {}", challenge2(&day1challenge1));
}
