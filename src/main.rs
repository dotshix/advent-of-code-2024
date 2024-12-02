mod day01a;
mod day01b;
mod day02a;
mod day02b;

fn main() {
    let day = std::env::args().nth(1).expect("Please specify a day (e.g., `1`)");
    match day.as_str() {
        "1a" => day01a::run(),
        "1b" => day01b::run(),
        "2a" => day02a::run(),
        "2b" => day02b::run(),
        _ => eprintln!("Day not implemented: {}", day),
    }
}
