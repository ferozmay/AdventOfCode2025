use std::{env, fs};

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    // let red_tiles: Vec<(i32, i32)> = data.lines().map(|line| let coordinates: (i32, i32) = line.split(",").map(|n| n.parse().unwrap().collect())).collect();
    println!("{:?}", data.lines());
}
