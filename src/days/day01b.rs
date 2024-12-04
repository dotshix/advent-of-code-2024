use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day01")
        .expect("Failed to read input file");

    println!("Solution for Day 1b: {}", solve(&input));
}

fn solve(input: &str) -> u32 {
    // grab input
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut map = HashMap::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();

        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    let mut res: u32 = 0;
    for num in list2 {
        *map.entry(num).or_insert(0) += 1;
    }

    for num in list1 {
        if let Some(&val) = map.get(&num){
            res += num as u32 * val;
        }
    }

    res
}
