mod day01a;
mod day01b;
mod day02a;
mod day02b;
mod day03a;
mod day03b;
mod day04a;
mod day04b;
mod day05a;
mod day05b;
mod day06a;
mod day06b;
mod day07a;
mod day07b;
mod day08a;
mod day08b;
mod day09a;
mod day09b;
mod day10a;
mod day10b;
mod day11a;
mod day11b;
mod day12a;
mod day12b;

pub fn run(day: &str) {
    match day {
        "1a" => day01a::run(),
        "1b" => day01b::run(),
        "2a" => day02a::run(),
        "2b" => day02b::run(),
        "3a" => day03a::run(),
        "3b" => day03b::run(),
        "4a" => day04a::run(),
        "4b" => day04b::run(),
        "5a" => day05a::run(),
        "5b" => day05b::run(),
        "6a" => day06a::run(),
        "6b" => day06b::run(),
        "7a" => day07a::run(),
        "7b" => day07b::run(),
        "8a" => day08a::run(),
        "8b" => day08b::run(),
        "9a" => day09a::run(),
        "9b" => day09b::run(),
        "10a" => day10a::run(),
        "10b" => day10b::run(),
        "11a" => day11a::run(),
        "11b" => day11b::run(),
        "12a" => day12a::run(),
        "12b" => day12b::run(),

        _ => eprintln!("Day not implemented: {}", day),
    }
}
