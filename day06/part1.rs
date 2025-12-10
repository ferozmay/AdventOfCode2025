use std::{env, fs};

// INCOMPLETE

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let lines: Vec<&str> = data.lines().collect();
    let symbols = lines[lines.len() - 1];

    for i in 0..lines.len() - 1 {
        // let vals = line.split().unwrap();
        let vals = line.split(" ").collect();
        println!("{}", lines[i]);
        println!();
    }
}
