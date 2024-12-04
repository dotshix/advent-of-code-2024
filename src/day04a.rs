pub fn run() {
    let input = std::fs::read_to_string("inputs/day04").expect("Failed to read input file");

    println!("Solution for Day 4a: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let input = input
        .lines() // collect lines
        .map(|line| line.chars().collect::<Vec<char>>()) // collect Vec<char>
        .collect::<Vec<Vec<char>>>(); // collcet into Vec<Vec<char>>

    let mut res = 0;

    let xmas = ['X', 'M', 'A', 'S'];

    for (i, line) in input.iter().enumerate() {
        for (j, &ch) in line.iter().enumerate() {
            if ch == 'X' {
                // right
                if j + xmas.len() <= line.len() && line[j..j + xmas.len()] == xmas {
                    res += 1;
                }

                // Left
                if j >= xmas.len() - 1 {
                    let start = j + 1 - xmas.len();
                    let test = &line[start..=j];
                    if test.iter().rev().eq(xmas.iter()) {
                        res += 1;
                    }
                }

                //up
                if i >= xmas.len() - 1
                    && input[i][j] == xmas[0]
                    && input[i - 1][j] == xmas[1]
                    && input[i - 2][j] == xmas[2]
                    && input[i - 3][j] == xmas[3]
                {
                    res += 1;
                }

                // down
                if i + xmas.len() <= input.len()
                    && input[i][j] == xmas[0]
                    && input[i + 1][j] == xmas[1]
                    && input[i + 2][j] == xmas[2]
                    && input[i + 3][j] == xmas[3]
                {
                    res += 1;
                }

                // up and right
                if i >= xmas.len() - 1 && j + xmas.len() <= line.len() {
                    let mut total = 0;
                    for x in 0..4 {
                        if input[i - x][j + x] == xmas[x] {
                            total += 1;
                        }
                    }

                    if total == 4 {
                        res += 1;
                    }
                }

                // down and right
                if i + xmas.len() <= input.len() && j + xmas.len() <= line.len() {
                    let mut total = 0;
                    for x in 0..4 {
                        if input[i + x][j + x] == xmas[x] {
                            total += 1;
                        }
                    }

                    if total == 4 {
                        res += 1;
                    }
                }

                // up and left
                if i >= xmas.len() - 1 && j >= xmas.len() - 1 {
                    let mut total = 0;
                    for x in 0..4 {
                        if input[i - x][j - x] == xmas[x] {
                            total += 1;
                        }
                    }

                    if total == 4 {
                        res += 1;
                    }
                }

                // down and left
                if i + xmas.len() <= input.len() && j >= xmas.len() - 1 {
                    let mut total = 0;
                    for x in 0..4 {
                        if input[i + x][j - x] == xmas[x] {
                            total += 1;
                        }
                    }

                    if total == 4 {
                        res += 1;
                    }
                }
            }
        }
    }

    res
}
