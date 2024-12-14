use std::collections::HashSet;

use itertools::Itertools;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day10").expect("Failed to read input file");
    println!("Solution for Day 10a: {}", solve(&input));
}

fn solve(input: &str) -> u32 {
    // let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()) .collect();
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect_vec();

    let mut res = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 0 {
                let mut seen = HashSet::new();
                res += dfs(&input, (i as isize, j as isize), 0, &mut seen);
            }
        }
    }
    res
}

fn dfs(input: &[Vec<u32>], (i, j): (isize, isize), curr: u32, seen_nine: &mut HashSet<(isize,isize)>) -> u32 {
    if !(i >= 0 && i < input.len() as isize && j >= 0 && j < input[0].len() as isize) {
        return 0;
    }

    if input[i as usize][j as usize] != curr {
        return 0;
    }

    if input[i as usize][j as usize] == 9 && curr == 9 {

        if seen_nine.contains(&(i,j)) {
            return 0;
        }

        seen_nine.insert((i,j));

        return 1;
    }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut count = 0;

    for (di, dj) in directions {
        count += dfs(input, (i + di, j + dj), curr + 1, seen_nine);
    }

    count
}
