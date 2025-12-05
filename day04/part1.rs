use std::env;
use std::fs;

// INCOMPLETE
fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible = 0;
    for r in 0..rows {
        for c in 0..cols {
            let mut rolls = 0;
            for rr in [-1_i32, 0, 1] {
                for cc in [-1_i32, 0, 1] {
                    let nr = ri + rr;
                    let nc = ci + cc;
                    if 0 <= rr && rr <= r && 0 <= cc && cc <= c {
                        if grid[r + rr][c + cc] == '@' {
                            rolls += 1;
                        }
                    }
                }
            }
            if rolls < 8 {
                accessible += 1
            }
        }
    }
    println!("Part 1: {}", accessible);
}
