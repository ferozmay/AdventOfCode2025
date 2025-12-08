use std::cmp::max;
use std::{env, fs};

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let (_ranges, _) = data.split_once("\n\n").unwrap();
    let mut ranges: Vec<(i64, i64)> = _ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    ranges.sort();
    let mut merged_range = Vec::new();
    let (mut curr_start, mut curr_end) = ranges[0];

    for (start, end) in ranges {
        if start <= curr_end {
            curr_end = max(curr_end, end);
        } else {
            merged_range.push((curr_start, curr_end));
            curr_start = start;
            curr_end = end;
        }
    }
    merged_range.push((curr_start, curr_end));
    let total: i64 = merged_range.iter().map(|(s, e)| e - s + 1).sum();
    println!("Part 2: {}", total);
}
