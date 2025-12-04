use std::env;
use std::fs;

// INCOMPLETE
fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    for line in data.lines() {
        let len = line.len();
        let (mut high1, mut high2, mut id) = (0, 0, 0);
        let mut sum = 0;

        for index in 0..len {
            
            for j in index..(len-12)


        }

    }
}
