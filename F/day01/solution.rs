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
    let mut part1 = 0;
    let mut part2 = 0;

    for line in data.lines() {
        let steps = line[1..].parse::<i32>().unwrap();

        if line.starts_with("L") {
            let dist_to_zero = (100 - dial_pos) % 100;
            let crossings = (steps + dist_to_zero) / 100;
            part2 += crossings;

            dial_pos = (dial_pos - steps).rem_euclid(100);
        } else {
            let crossings = (dial_pos + steps) / 100;
            part2 += crossings;
            dial_pos = (dial_pos + steps).rem_euclid(100);
        }

        if dial_pos == 0 {
            part1 += 1;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
