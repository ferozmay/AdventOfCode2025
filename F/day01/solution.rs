use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let f = &args[1];
    let data = fs::read_to_string(f).expect("Failed to read file");

    let mut dial_pos = 50;
    let mut count = 0;

    for line in data.lines() {
        if line.starts_with("L") {
            let steps = line[1..].parse::<i32>().unwrap();
            dial_pos = (dial_pos - steps).rem_euclid(100);
        } else {
            let steps = line[1..].parse::<i32>().unwrap();
            dial_pos = (dial_pos + steps).rem_euclid(100);
        }

        if dial_pos == 0 {
            count += 1;
        }
    }
    println!("Final dial position: {}", dial_pos);
    println!("Part 1: {}", count);
}
