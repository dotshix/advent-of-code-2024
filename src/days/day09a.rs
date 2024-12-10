use itertools::Itertools;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day09").expect("Failed to read input file");
    println!("Solution for Day 9a: {}", solve(&input));
}

fn solve(input: &str) -> u64 {
    let input = input.chars();

    let mut file: Vec<String> = vec![];

    let mut id = 0;

    let mut res = 0;

    for chunk in &input.chunks(2) {
        let chunk: Vec<_> = chunk.collect();

        let mut id_vec = vec![id.to_string(); chunk[0].to_digit(10).unwrap() as usize];
        file.append(&mut id_vec);

        id += 1;

        if let Some(count) = chunk.get(1).and_then(|&c| c.to_digit(10)) {
            let mut dots = vec![".".to_string(); count as usize];
            file.append(&mut dots);
        }
    }

    move_blocks(&mut file);

    id = 0;

    // do checksum
    for block in file {
        let block = block.parse::<u64>();

        if let Ok(num) = block {
            res += id * num;
        }

        id += 1;
    }

    res
}

fn move_blocks(file: &mut Vec<String>) {
    // find left empty block
    let mut l = file.iter().position(|i| i == ".").unwrap();
    let mut r = file.len() - 1;

    while l < r {
        let lstr = &file[l];
        let rstr = &file[r];

        if lstr == "." && rstr != "." {
            file.swap(l, r);
            l += 1;
            r -= 1;
        } else if lstr != "." {
            l += 1;
        } else if rstr == "." {
            r -= 1;
        }
    }
}
