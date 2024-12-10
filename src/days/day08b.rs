use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = std::fs::read_to_string("inputs/day08").expect("Failed to read input file");
    println!("Solution for Day 8b: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let mut antinodes_set = HashSet::new();

    let mut char_to_location: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch != '.' {
                char_to_location.entry(ch).or_default().push((i, j));
            }
        }
    }

    for (_, val) in char_to_location.iter() {
        let gens = generate_combinations(val);

        for gen in gens {
            let (x1, y1) = (gen[0].0 as i32, gen[0].1 as i32);
            let (x2, y2) = (gen[1].0 as i32, gen[1].1 as i32);

            // Compute the two antinodes for this pair.
            let antinodes = find_antinodes(x1, y1, x2, y2, 50);

            for ant in antinodes {
                // println!("{:?}", ant);
                let (ax, ay) = ant;
                // Check if antinode is within the bounds of the map.
                if !out_of_bounds((ax as isize, ay as isize), &map) {
                    antinodes_set.insert((ax, ay));
                }
            }
        }
    }

    antinodes_set.len() as i32
}

fn generate_combinations(points: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    points
        .iter()
        .combinations(2) // Generate all combinations of 2 pairs.
        .map(|pair| pair.into_iter().cloned().collect())
        .collect()
}

fn find_antinodes(x1: i32, y1: i32, x2: i32, y2: i32, range: i32) -> Vec<(i32, i32)> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut antinodes = Vec::new();

    for k in -range..=range {
        let antinode_x = x1 + dx * k;
        let antinode_y = y1 + dy * k;
        antinodes.push((antinode_x, antinode_y));
    }

    antinodes
}

fn out_of_bounds((i, j): (isize, isize), map: &[Vec<char>]) -> bool {
    i < 0 || j < 0 || i >= map.len() as isize || j >= map[0].len() as isize
}
