use std::collections::HashSet;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day06").expect("Failed to read input file");
    println!("Solution for Day 6a: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let map: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    // Find the start position
    let mut start = None;
    for (i, line) in map.iter().enumerate() {
        if let Some((j, _)) = line.iter().enumerate().find(|&(_, &ch)| ch == b'^') {
            start = Some((i as isize, j as isize)); // Convert to isize for walking
            break;
        }
    }

    let mut current_pos = start.expect("Start position not found!");

    // Walk
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction_idx = 0; // Start with initial direction
    let mut res = 0;
    let mut seen = HashSet::new();

    seen.insert(current_pos);

    loop {
        let (di, dj) = directions[direction_idx % directions.len()];
        let next_pos = (current_pos.0 + di, current_pos.1 + dj);

        if out_of_bounds(next_pos, &map) {
            res += if !seen.contains(&next_pos) {
                seen.insert(next_pos);
                1
            } else {
                0
            };
            break;
        }

        let cell = map[next_pos.0 as usize][next_pos.1 as usize];
        match cell {
            b'.' => {
                current_pos = next_pos;
                res += if !seen.contains(&current_pos) {
                    seen.insert(current_pos);
                    1
                } else {
                    0
                };
            }
            b'#' => {
                // hit a wall, don't move but rotate direction
                direction_idx += 1;
            }
            _ => {
                break;
            }
        }
    }

    res
}

fn out_of_bounds((i, j): (isize, isize), map: &[&[u8]]) -> bool {
    i < 0 || j < 0 || i >= map.len() as isize || j >= map[0].len() as isize
}
