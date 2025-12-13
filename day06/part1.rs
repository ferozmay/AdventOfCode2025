use std::{env, fs};

// INCOMPLETE

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let lines: Vec<&str> = data.lines().collect();
    let n = lines.len() - 1;
    let symbols = lines[n];

    println!("{}", n);
    println!("{}", symbols);

    // for i in 0..n {
    //     let vals = lines[i].split(" ").collect();
    //     println!("{}", lines[i]);
    //     println!();
    // }
}
