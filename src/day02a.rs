pub fn run() {
    let input = std::fs::read_to_string("inputs/day02").expect("Failed to read input file");

    println!("Solution for Day 2: {}", solve(&input));
}

fn solve_helper(nums: Vec<i32>) -> bool {
    let mut direction: Option<i32> = None;

    let mut prev: Option<i32> = None;
    for num in nums {
        if let Some(p) = prev {
            let diff = num - p;

            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            let curr_direction = if diff > 0 { 1 } else { -1 };

            if direction == None {
                direction = Some(curr_direction);
            } else if let Some(d) = direction {
                if d != curr_direction {
                    return false;
                }
            }
        }

        prev = Some(num);
    }

    true
}

fn solve(input: &str) -> i32 {
    let mut res = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if solve_helper(nums) {
            res += 1;
        }
    }

    res
}
