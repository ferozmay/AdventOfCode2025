use std::env;
use std::fs;

fn is_invalid_id(id: i64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        return false;
    }
    let (split1, split2) = id_str.split_at(id_str.len() / 2);
    split1 == split2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(&args[1]).unwrap();
    let mut sum: i64 = 0;

    for range in data.split(",") {
        let parts: Vec<&str> = range.split('-').collect();
        let (low, high) = (parts[0], parts[1]);

        if (low.len() == high.len()) && low.len() % 2 != 0 {
            continue;
        }

        let low_int: i64 = low.parse().unwrap();
        let high_int: i64 = high.parse().unwrap();

        for n in low_int..high_int + 1 {
            if is_invalid_id(n) {
                sum += n;
            }
        }
    }
    println!("Part 1: {}", sum);
}
