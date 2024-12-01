mod day01a;

fn main() {
    let day = std::env::args().nth(1).expect("Please specify a day (e.g., `1`)");
    match day.as_str() {
        "1a" => day01a::run(),
        _ => eprintln!("Day not implemented: {}", day),
    }
}
