use std::{env, fs};

fn euclidean_dist_squared(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)
}

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let points: Vec<(i64, i64, i64)> = data
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split(",").map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..points.len() - 1 {
        for j in (i + 1)..points.len() {
            let dist = euclidean_dist_squared(points[i], points[j]);
            edges.push((dist, i, j));
        }
    }
    edges.sort();
    const CONNECTED_PAIRS: usize = 10;
    for (dist, i, j) in edges[0..CONNECTED_PAIRS].to_vec() {
        println!(
            "The distance between point {} and point {} is {}",
            i, j, dist
        );
    }
    // println!("{:?}", edges);
}
