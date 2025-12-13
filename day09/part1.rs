use std::{cmp, env, fs};

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let red_tiles: Vec<(i64, i64)> = data
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().trim().parse().unwrap();
            let y: i64 = parts.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect();

    let n = red_tiles.len();
    let mut max_area: i64 = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let curr_area = width * height;
            max_area = cmp::max(max_area, curr_area);
        }
    }
    println!("Max area: {}", max_area);
}
