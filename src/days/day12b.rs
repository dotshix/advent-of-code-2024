use std::collections::{HashSet, VecDeque};

pub fn run() {
    let input = std::fs::read_to_string("inputs/day12").expect("Failed to read input file");
    println!("Solution for Day 12b: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|ch| ch.chars().collect()).collect();

    let mut seen = HashSet::new();
    let mut res = 0;
    for (x, line) in map.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if !seen.contains(&(x as isize, y as isize)) {
                let (area, perimeter) = bfs(&(x as isize, y as isize), ch, &map, &mut seen);
                // println!("{ch},{area},{perimeter}");
                res += area * perimeter;
            }
        }
    }

    res
}

fn bfs(
    start: &(isize, isize),
    ch: &char,
    map: &Vec<Vec<char>>,
    seen: &mut HashSet<(isize, isize)>,
) -> (i32, i32) {
    // if seen break
    if seen.contains(start) {
        return (0, 0);
    }

    let rows = map.len();
    let cols = map[0].len();
    let mut area = 0;
    let mut perimeter = 0;

    let mut bfs = VecDeque::from([*start]);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    seen.insert(*start);
    while !bfs.is_empty() {
        let (x, y) = bfs.pop_front().unwrap();

        // println!("{x},{y}");

        area += 1;
        perimeter += get_sides(map, &(x, y), ch);
        // println!("Perimeter: {perimeter}");
        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0
                && ny >= 0
                && nx < rows as isize
                && ny < cols as isize
                && map[nx as usize][ny as usize] == *ch
                && !seen.contains(&(nx, ny))
            {
                seen.insert((nx, ny));
                bfs.push_back((nx, ny));
            }
        }
    }

    (area, perimeter)
}

fn get_sides(map: &[Vec<char>], start: &(isize, isize), ch: &char) -> i32 {
    let rows = map.len() as isize;
    let cols = map.first().map_or(0, |row| row.len()) as isize;

    let (x, y) = *start;
    let mut sides = 0;

    // Helper to check bounds and return the character if valid
    let get_char = |r: isize, c: isize| -> Option<char> {
        if r >= 0 && r < rows && c >= 0 && c < cols {
            Some(map[r as usize][c as usize])
        } else {
            None
        }
    };

    // Directions: (delta_x, delta_y, diag_x, diag_y)
    let directions = [
        ((1, 0), (0, -1), (1, -1)),   // DOWN-LEFT
        ((-1, 0), (0, -1), (-1, -1)), // UP-LEFT
        ((-1, 0), (0, 1), (-1, 1)),   // UP-RIGHT
        ((1, 0), (0, 1), (1, 1)),     // DOWN-RIGHT
    ];

    for &((dx, dy1), (dx2, dy2), (diag_x, diag_y)) in &directions {
        let down = get_char(x + dx, y + dy1);
        let side = get_char(x + dx2, y + dy2);
        let diag = get_char(x + diag_x, y + diag_y);

        match (down, side, diag) {
            (Some(d), Some(s), Some(diag)) if d == *ch && s == *ch && diag != *ch => sides += 1,
            (Some(d), Some(s), _) if d != *ch && s != *ch => sides += 1,
            _ => {}
        }

        // Handle out-of-bounds edge cases for each direction
        if down.is_none() && side.is_none() {
            sides += 1;
        } else if down.is_none() && side.map_or(false, |s| s != *ch) {
            sides += 1;
        } else if side.is_none() && down.map_or(false, |d| d != *ch) {
            sides += 1;
        }
    }

    sides
}
