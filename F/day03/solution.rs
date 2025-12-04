use std::env;
use std::fs;

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut sum = 0;
    for line in data.lines() {
        let len = line.len();
        let (mut high1, mut high2, mut id) = (0, 0, 0);
        for index in 0..(len - 1) {
            let digit = line.chars().nth(index).unwrap().to_digit(10).unwrap();
            if digit > high1 {
                high1 = digit;
                id = index;
            }
        }
        for j in (id + 1)..(len) {
            let digit = line.chars().nth(j).unwrap().to_digit(10).unwrap();
            if digit > high2 {
                high2 = digit;
            }
        }
        sum += 10 * high1 + high2;
    }
    println!("Part 1: {}", sum);
}
