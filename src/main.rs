mod days;

fn main() {
    days::run(&std::env::args().nth(1).expect("Please specify a day (e.g., `1a`)"));
}
