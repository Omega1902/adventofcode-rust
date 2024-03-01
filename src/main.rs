use clap::Parser;

/// Advent of Code resolver for 2023
#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    /// Specify number of day to solve between 1 and 25. Default will solve all days
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,
}

fn run_day(day: u8, verbose: bool) {
    match day {
        1 => adventofcode_rust::day01::main(),
        2 => adventofcode_rust::day02::main(),
        3 => adventofcode_rust::day03::main(),
        4 => adventofcode_rust::day04::main(),
        5 => adventofcode_rust::day05::main(),
        6 => adventofcode_rust::day06::main(),
        7 => adventofcode_rust::day07::main(),
        8 => adventofcode_rust::day08::main(),
        9 => adventofcode_rust::day09::main(),
        10 => adventofcode_rust::day10::main(verbose),
        11 => adventofcode_rust::day11::main(),
        i => println!("Day {i} is not (yet) available"),
    }
}

fn main() {
    let args = Args::parse();
    let max_day = 11;
    match args.day {
        None => {
            for day in 1..=max_day {
                run_day(day, false);
            }
        }
        Some(1..=25) => run_day(args.day.unwrap(), true),
        Some(i) => println!("Day {i} does not exist in AdventOfCode"),
    }
}
