use itertools::Itertools;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day10").expect("Failed to read input file");
    println!("Solution for Day 10b: {}", solve(&input));
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
                res += dfs(&input, (i as isize, j as isize), 0);
            }
        }
    }
    res
}

fn dfs(input: &[Vec<u32>], (i, j): (isize, isize), curr: u32) -> u32 {
    if !(i >= 0 && i < input.len() as isize && j >= 0 && j < input[0].len() as isize) {
        return 0;
    }

    if input[i as usize][j as usize] != curr {
        return 0;
    }

    if input[i as usize][j as usize] == 9 && curr == 9 {
        return 1;
    }

    println!("{}, {curr}", input[i as usize][j as usize]);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];


    let mut new_dir: Vec<(isize, isize)> = Vec::new();

    for (di, dj) in directions {
        new_dir.push((i + di, j + dj));
    }
    dfs(input, new_dir[0], curr + 1)
        + dfs(input, new_dir[1], curr + 1)
        + dfs(input, new_dir[2], curr + 1)
        + dfs(input, new_dir[3], curr + 1)
}
