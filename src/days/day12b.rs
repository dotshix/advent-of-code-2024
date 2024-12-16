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
                let (area, perimeter) = bfs(&(x as isize, y as isize), &ch, &map, &mut seen);
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
    if seen.contains(&start) {
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
        perimeter += get_perimeter(&map, &(x, y), &ch);
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

fn get_perimeter(map: &Vec<Vec<char>>, start: &(isize, isize), ch: &char) -> i32 {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let mut perimeter = 0;
    let (x, y) = *start;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dx, dy) in directions.iter() {
        let nx = x + dx;
        let ny = y + dy;

        if nx < 0 || ny < 0 || nx >= rows || ny >= cols {
            // Out of bounds, add to perimeter
            perimeter += 1;
        } else if map[nx as usize][ny as usize] != *ch {
            // Different character, add to perimeter
            perimeter += 1;
        }
    }

    perimeter
}
