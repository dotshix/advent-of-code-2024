use std::collections::HashMap;

use itertools::Itertools;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day09").expect("Failed to read input file");
    println!("Solution for Day 9b: {}", solve(&input));
}

fn solve(input: &str) -> u64 {
    let input = input.chars();

    let mut file: Vec<String> = vec![];

    let mut id = 0;

    let mut map = HashMap::new();

    for chunk in &input.chunks(2) {
        let chunk: Vec<_> = chunk.collect();

        let id_to_string = id.to_string();
        let chunk_to_string = chunk[0].to_digit(10).unwrap();
        let mut id_vec = vec![id_to_string.clone(); chunk_to_string as usize];
        file.append(&mut id_vec);
        map.insert(id_to_string, chunk_to_string);

        id += 1;

        if let Some(count) = chunk.get(1).and_then(|&c| c.to_digit(10)) {
            let mut dots = vec![".".to_string(); count as usize];
            file.append(&mut dots);
        }
    }

    move_blocks(&mut file, &mut map);
    checksum(&file)
}

fn move_blocks(file: &mut [String], map: &mut HashMap<String, u32>) {
    // println!("Initial state: {:?}", file);

    let mut rpointer = file.iter().rposition(|i| i != ".").unwrap();

    let mut lpointer = file.iter().position(|i| i == ".").unwrap();

    let mut last_num: i32 = file[rpointer].parse().unwrap();

    while last_num != 0 {
        // find how many dots
        let count = file[lpointer..].iter().take_while(|&i| i == ".").count();

        // println!("Dots found: {count}");

        if let Some(num_count) = map.get_mut(&file[rpointer]) {
            if count >= *num_count as usize && lpointer < rpointer {
                while *num_count != 0 {
                    file.swap(rpointer, lpointer);
                    *num_count -= 1;
                    lpointer += 1;
                    rpointer -= 1;
                }

                lpointer = file.iter().position(|i| i == ".").unwrap();
                last_num -= 1;
                rpointer = file
                    .iter()
                    .rposition(|i| *i == last_num.to_string())
                    .unwrap();
                last_num = file[rpointer].parse().unwrap();
            } else {
                lpointer += count;

                if let Some(pos) = file[lpointer..].iter().position(|i| i == ".") {
                    lpointer += pos;
                } else {
                    // // Handle the case where no "." is found
                    lpointer = file.iter().position(|i| i == ".").unwrap();

                    last_num -= 1;
                    rpointer = file
                        .iter()
                        .rposition(|i| *i == last_num.to_string())
                        .unwrap();
                    last_num = file[rpointer].parse().unwrap();
                }
            }
        }
    }
}

// do checksum
fn checksum(file: &[String]) -> u64 {
    let mut res = 0;

    for (id, block) in file.iter().enumerate() {
        if let Ok(num) = block.parse::<u64>() {
            res += id as u64 * num;
        }
    }

    res
}
