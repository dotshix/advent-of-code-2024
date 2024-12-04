pub fn run() {
    let input = std::fs::read_to_string("inputs/day04").expect("Failed to read input file");

    println!("Solution for Day 4b: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let input = input
        .lines() // collect lines
        .map(|line| line.chars().collect::<Vec<char>>()) // collect Vec<char>
        .collect::<Vec<Vec<char>>>(); // collcet into Vec<Vec<char>>

    let mut res = 0;

    let xmas1 = [['M', '.', 'S'],
                ['.', 'A', '.'],
                ['M', '.', 'S']];

    let xmas2 = [['S', '.', 'S'],
                ['.', 'A', '.'],
                ['M', '.', 'M']];

    let xmas3 = [['M', '.', 'M'],
                ['.', 'A', '.'],
                ['S', '.', 'S']];

    let xmas4 = [['S', '.', 'M'],
                ['.', 'A', '.'],
                ['S', '.', 'M']];



    for i in 0..input.len() - 2 {
        for j in 0..input[i].len() - 2 {
            // Using fixed-size arrays
            let slice = [
                [input[i][j], '.', input[i][j + 2]],
                ['.', input[i + 1][j + 1], '.'],
                [input[i + 2][j], '.', input[i + 2][j + 2]],
            ];

            if slice == xmas1 || slice == xmas2 || slice == xmas3 || slice == xmas4 {
                res += 1;
            }
        }
    }

    res
}
