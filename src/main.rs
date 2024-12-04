mod day01a;
mod day01b;
mod day02a;
mod day02b;
mod day03a;
mod day03b;
mod day04a;

fn main() {
    let day = std::env::args().nth(1).expect("Please specify a day (e.g., `1`)");
    match day.as_str() {
        "1a" => day01a::run(),
        "1b" => day01b::run(),
        "2a" => day02a::run(),
        "2b" => day02b::run(),
        "3a" => day03a::run(),
        "3b" => day03b::run(),
        "4a" => day04a::run(),
        _ => eprintln!("Day not implemented: {}", day),
    }
}
