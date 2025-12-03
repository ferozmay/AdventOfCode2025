use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let data = fs::read_to_string(file).unwrap();
    let mut sum: i64 = 0;

    for range in data.split(",") {
        let low = range.split("-").next().unwrap().parse::<String>().unwrap();
        let high = range.split("-").nth(1).unwrap().parse::<String>().unwrap();

        if (low.len() == high.len()) && low.len() % 2 != 0 {
            continue;
        }

        let low_int = low.parse::<i64>().unwrap();
        let high_int = high.parse::<i64>().unwrap();

        for number in low_int..high_int + 1 {
            let num_str = number.to_string();
            let (split1, split2) = num_str.split_at(num_str.len() / 2);
            if split1 == split2 {
                sum += number;
            }
        }
    }
    println!("Part 1: {}", sum);
}
