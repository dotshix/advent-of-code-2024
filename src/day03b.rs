use regex::Regex;

pub fn run() {
    let input = std::fs::read_to_string("inputs/day03").expect("Failed to read input file");

    println!("Solution for Day 3b: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let re = Regex::new(r"(?:(mul)\((\d+),(\d+)\))|(?:(do)\(\))|(?:(don't)\(\))").unwrap();
    let mut res = 0;
    let mut flag = true;
    for cap in re.captures_iter(input) {

        if let Some(_) = cap.get(1) {
            if flag == true {
                let num1 = &cap[2];
                let num2 = &cap[3];

                res += num1.to_string().parse::<i32>().unwrap()
                    * num2.to_string().parse::<i32>().unwrap();
            }
        } else if let Some(_) = cap.get(5) {
            flag = false;
        } else if let Some(_) = cap.get(4) {
            flag = true;
        }
    }

    res
}
