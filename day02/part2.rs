use std::env;
use std::fs;

fn is_invalid_id(id: i64) -> bool {
    let id_str = id.to_string();
    let n = id_str.len();
    for i in 1..=n / 2 {
        let substr = &id_str[0..i];
        let repeats = n / i;
        if substr.repeat(repeats) == id_str {
            return true;
        }
    }
    return false;
}

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut sum: i64 = 0;

    for range in data.split(",") {
        let parts: Vec<&str> = range.split('-').collect();
        let (low, high) = (parts[0], parts[1]);

        let low_int: i64 = low.parse().unwrap();
        let high_int: i64 = high.parse().unwrap();

        for n in low_int..=high_int {
            if is_invalid_id(n) {
                sum += n;
            }
        }
    }
    println!("Part 2: {}", sum);
}
