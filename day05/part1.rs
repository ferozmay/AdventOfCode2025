use std::{env, fs};

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let parts: Vec<&str> = data.split("\n\n").collect();

    let mut range_start: Vec<i64> = Vec::new();
    let mut range_end: Vec<i64> = Vec::new();

    for line in parts[0].lines() {
        let nums: Vec<i64> = line.split('-').map(|n| n.parse().unwrap()).collect();
        range_start.push(nums[0]);
        range_end.push(nums[1]);
    }

    let numbers: Vec<i64> = parts[1].lines().map(|n| n.parse().unwrap()).collect();
    let mut count = 0;
    for num in numbers {
        for i in 0..range_end.len() {
            if num == range_end[i] {
                count += 1;
                break;
            } else if num < range_end[i] {
                if num >= range_start[i] {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", count);
}
