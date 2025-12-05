use std::env;
use std::fs;

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut sum = 0;

    for line in data.lines() {
        let len = line.len();
        let mut index = 0;
        let mut num = String::new();
        let mut to_pick = 12;

        while to_pick > 0 {
            let end = len - to_pick;
            let mut high = 0;
            let mut found = index;

            for i in index..=end {
                let digit = line.chars().nth(i).unwrap().to_digit(10).unwrap();
                if digit > high {
                    high = digit;
                    found = i;
                }
            }
            num.push_str(&high.to_string());
            index = found + 1;
            to_pick -= 1;
        }
        sum += num.parse::<i64>().unwrap();
    }
    println!("Part 2: {}", sum);
}
