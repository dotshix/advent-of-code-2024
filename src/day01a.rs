pub fn run() {
    let input = std::fs::read_to_string("inputs/day01")
        .expect("Failed to read input file");

    println!("Solution for Day 1: {}", solve(&input));
}

fn solve(input: &str) -> u32 {
    // grab input
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();

        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    list1.sort();
    list2.sort();

    let mut res = 0;
    for (left, right) in list1.iter().zip(list2.iter()) {
        res += (left - right).abs() as u32;
    }

    res
}
