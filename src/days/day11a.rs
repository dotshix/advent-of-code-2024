use itertools::Itertools;
use std::collections::LinkedList;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day11").expect("Failed to read input file");
    println!("Solution for Day 11a: {}", solve(&input));
}

fn solve(input: &str) -> usize {
    let input: Vec<String> = input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect_vec();

    let mut list: LinkedList<String> = input.into_iter().collect();

    let mut i = 0;
    while i < 25 {
        let mut result = LinkedList::new();

        while let Some(value) = list.pop_front() {
            // result.push_back(value);

            let size = value.len();

            if value == "0".to_string() {
                result.push_back("1".to_string());
            } else if size % 2 == 0 {
                let left = &value[..size / 2];
                let right = adjust_slice(&value);

                result.push_back(left.to_string());
                result.push_back(right.to_string());
            } else {
                let mut num: u64 = value.parse().unwrap();

                num *= 2024;

                result.push_back(num.to_string());
            }
        }

        // Reassign the modified list back to the original variable
        list = result;

        i += 1;
    }

    // list.iter().map(|num| num.parse::<u64>().unwrap()).sum()
    list.len()
}

fn adjust_slice(value: &str) -> &str {
    let size = value.len();
    let mut right = &value[size / 2..];

    // Remove leading zeros only if the slice length is greater than 1
    if right.len() > 1 {
        right = right.trim_start_matches('0');
        // If the slice becomes empty after removing zeros, it should be "0"
        if right.is_empty() {
            right = "0";
        }
    }

    right
}
