use std::collections::HashMap;
use std::collections::HashSet;
use topo_sort::{SortResults, TopoSort};

pub fn run() {
    let input = std::fs::read_to_string("inputs/day05").expect("Failed to read input file");
    println!("Solution for Day 5a: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let lines = input.trim().lines();

    let mut forward = HashMap::new();

    let mut res = 0;
    for line in lines {
        let elements = parse_elements(line);

        if elements.len() == 2 {
            update_rules(&mut forward, &elements);
        } else if elements.len() > 2 {
            let is_correct = is_sequence_correct(&elements, &forward);

            if !is_correct {
                let set_elements: HashSet<_> = elements.iter().cloned().collect();
                match create_correct_sequence(&forward, &elements, &set_elements) {
                    Some(correct_sequence) => {
                        res += correct_sequence[correct_sequence.len() / 2]
                            .parse::<i32>()
                            .unwrap_or(0)
                    }
                    None => println!("Failed to generate a valid sequence due to a cycle."),
                }
            }
        }
    }

    res
}

// gets it in reverse, oh well
fn create_correct_sequence<'a>(
    forward: &HashMap<&'a str, Vec<&'a str>>,
    elements: &[&'a str],
    set_elements: &HashSet<&'a str>,
) -> Option<Vec<&'a str>> {
    let mut topo_sort = TopoSort::with_capacity(elements.len());

    for &val in elements {
        let mut dependencies = Vec::new();
        if let Some(points_to) = forward.get(val) {
            dependencies.extend(points_to.iter().filter(|&&dep| set_elements.contains(dep)));
        }

        topo_sort.insert(val, dependencies);
    }

    match topo_sort.into_vec_nodes() {
        SortResults::Full(nodes) => Some(nodes),
        SortResults::Partial(partial) => {
            println!("Cycle detected among nodes: {:?}", partial);
            None
        }
    }
}


// Parsed based on '|' or ','
fn parse_elements(line: &str) -> Vec<&str> {
    line.split(['|', ',']).filter(|s| !s.is_empty()).collect()
}

// Update the forward rules
// removed backwards checking
fn update_rules<'a>(forward: &mut HashMap<&'a str, Vec<&'a str>>, elements: &[&'a str]) {
    forward.entry(elements[0]).or_default().push(elements[1]);
}

// is correct based on the rules
fn is_sequence_correct(elements: &[&str], forward: &HashMap<&str, Vec<&str>>) -> bool {
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

    correct
}
