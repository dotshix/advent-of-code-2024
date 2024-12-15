use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day11").expect("Failed to read input file");
    println!("Solution for Day 11b: {}", solve(&input));
}

fn solve(input: &str) -> usize {
    let stones: Vec<String> = input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect_vec();

    let steps = 75;
    let mut memo = HashMap::new();

    let mut total = 0;
    for stone in stones {
        total += dfs(&stone, steps, &mut memo);
    }

    total
}

fn dfs(number: &str, steps_left: usize, memo: &mut HashMap<(String, usize), usize>) -> usize {
    if steps_left == 0 {
        return 1; // exactly one stone
    }

    let key = (number.to_string(), steps_left);

    // retrn if in memo
    if let Some(&val) = memo.get(&key) {
        return val;
    }

    let res;

    // go through rules
    if number == "0" {
        res = dfs("1", steps_left - 1, memo);
    } else if number.len() % 2 == 0 {
        let left_num = &number[0..number.len() / 2];
        let right_num = adjust_slice(number);

        res = dfs(left_num, steps_left - 1, memo) + dfs(right_num, steps_left - 1, memo);
    } else {
        let new_num = number.parse::<u64>().unwrap() * 2024;

        res = dfs(&new_num.to_string(), steps_left - 1, memo);
    }

    memo.insert(key, res);
    res
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
