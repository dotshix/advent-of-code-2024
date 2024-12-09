use itertools::Itertools;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day07").expect("Failed to read input file");
    println!("Solution for Day 7a: {}", solve(&input));
}

fn solve(input: &str) -> u64 {
    let mut res = 0;

    let input: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split(": ").collect())
        .collect();

    for line in input {
        let total: u64 = line[0].parse().unwrap();
        let combos = generate_combinations(line[1]);

        for combo in combos {
            let mut combo = combo.into_iter();

            let replaced: String = line[1]
                .chars()
                .map(|c| {
                    if c == ' ' {
                        combo.next().unwrap() // Use the next replacement character
                    } else {
                        c // Keep the original character
                    }
                })
                .collect();

            if total == eval(&replaced){
                res += total;
                break;
            }
        }
    }

    res
}

fn generate_combinations(expr: &str) -> Vec<Vec<char>> {
    let base_set = vec!['*', '+'];

    // how many do we need
    let count = expr.chars().filter(|c| *c == ' ').count();

    let cart = vec![base_set; count];

    let cartesian_product = cart
        .iter()
        .map(|inner| inner.iter().cloned())
        .multi_cartesian_product();

    let res = cartesian_product.collect_vec();

    res
}

fn eval(expr: &str) -> u64 {
    let mut stk = Vec::new();
    let mut last_op: Option<char> = None;
    let ops = ['+', '*'];
    let mut num: u64 = 0;

    for ch in expr.chars() {
        if ch.is_digit(10) {
            num = num * 10 + ch.to_digit(10).unwrap() as u64;
        } else if ops.contains(&ch) && !last_op.is_none() && last_op.unwrap() == '*' {
            let last_num = stk.pop().unwrap();

            stk.push(last_num * num);

            last_op = Some(ch);
            num = 0;
        } else if ops.contains(&ch) && !last_op.is_none() && last_op.unwrap() == '+' {

            let last_num = stk.pop().unwrap();
            stk.push(num + last_num);
            last_op = Some(ch);
            num = 0;
        } else {
            stk.push(num);
            last_op = Some(ch);
            num = 0;
        }
    }

    match last_op {
        Some('*') => {
            let last_num = stk.pop().unwrap();
            stk.push(num * last_num);
        }

        Some('+') => stk.push(num),

        _ => println!("Error, no last op"),
    }

    stk.iter().sum()
}
