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
        _ => eprintln!("Day not implemented: {}", day),
    }
}
