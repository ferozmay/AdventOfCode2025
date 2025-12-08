use std::collections::HashMap;
use std::{env, fs};

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let cols = lines[0].len();

    let mut beams = HashMap::new();
    beams.insert(start_col, 1);

    for line in lines {
        let mut next: HashMap<i32, u64> = HashMap::new();

        for (&col, &count) in &beams {
            if col < 0 || col >= cols as i32 {
                continue;
            }

            if line.as_bytes()[col as usize] == b'^' {
                *next.entry(col - 1).or_insert(0) += count;
                *next.entry(col + 1).or_insert(0) += count;
            } else {
                *next.entry(col).or_insert(0) += count;
            }
        }
        println!("{:?}", next);
        beams = next;
    }

    let mut total: u64 = beams.values().sum();

    println!("Part 2: {:?}", total);
}
