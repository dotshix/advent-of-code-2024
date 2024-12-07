use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day05").expect("Failed to read input file");
    println!("Solution for Day 5a: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let lines = input.trim().lines();

    let mut forward = HashMap::new();
    let mut backwards = HashMap::new();

    let mut res = 0;
    for line in lines {
        let elements = parse_elements(line);

        if elements.len() == 2 {
            update_rules(&mut forward, &mut backwards, &elements);
        } else if elements.len() > 2 {
            if is_sequence_correct(&elements, &forward, &backwards) {
                res += get_middle_value(&elements);
            }
        }
    }

    res
}

// Parsed based on '|' or ','
fn parse_elements(line: &str) -> Vec<&str> {
    line.split(['|', ','])
        .filter(|s| !s.is_empty())
        .collect()
}

// Update the forward and backwards rules
fn update_rules<'a>(
    forward: &mut HashMap<&'a str, Vec<&'a str>>,
    backwards: &mut HashMap<&'a str, Vec<&'a str>>,
    elements: &[&'a str],
) {
    forward
        .entry(elements[0])
        .or_default()
        .push(elements[1]);

    backwards
        .entry(elements[1])
        .or_default()
        .push(elements[0]);
}

// is correct based on the rules
fn is_sequence_correct(
    elements: &[&str],
    forward: &HashMap<&str, Vec<&str>>,
    backwards: &HashMap<&str, Vec<&str>>,
) -> bool {
    let mut idx = HashMap::new();
    let mut correct = true;

    for (i, &num) in elements.iter().enumerate() {
        idx.insert(num, i);
    }

    for (i, &num) in elements.iter().enumerate() {
        if let Some(fvals) = forward.get(num) {
            for val in fvals {
                if let Some(&v) = idx.get(val) {
                    if i > v {
                        correct = false;
                    }
                }
            }
        }
    }

    for (i, &num) in elements.iter().enumerate() {
        if let Some(bvals) = backwards.get(num) {
            for val in bvals {
                if let Some(&v) = idx.get(val) {
                    if i < v {
                        correct = false;
                    }
                }
            }
        }
    }

    correct
}

// Get the middle value of the elements array and parse it as an integer.
fn get_middle_value(elements: &[&str]) -> i32 {
    let middle_index = elements.len() / 2;
    elements[middle_index].parse::<i32>().unwrap()
}
