use std::collections::HashSet;
use std::{env, fs};

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let cols = lines[0].len();

    let mut beams = HashSet::new();
    for c in 0..cols {
        if lines[0].as_bytes()[c] == b'S' {
            beams.insert(c as i32);
        }
    }
    let mut splits = 0;
    for line in lines {
        let mut next_beams = HashSet::new();

        for &col in &beams {
            if col < 0 || col >= cols as i32 {
                continue;
            }
            let cell = line.as_bytes()[col as usize];
            if cell == b'^' {
                splits += 1;
                next_beams.insert(col - 1);
                next_beams.insert(col + 1);
            } else {
                next_beams.insert(col);
            }
        }
        beams = next_beams;
    }
    println!("Part 1: {}", splits);
}
