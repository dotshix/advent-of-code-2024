use regex::Regex;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day03").expect("Failed to read input file");

    println!("Solution for Day 3a: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let mut res = 0;
    for cap in re.captures_iter(input) {
        let num1 = &cap[1];
        let num2 = &cap[2];

        res += num1.to_string().parse::<i32>().unwrap() * num2.to_string().parse::<i32>().unwrap();
    }

    res
}
