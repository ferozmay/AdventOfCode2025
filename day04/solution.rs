use std::env;
use std::fs;

fn main() {
    let data = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let mut grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut accessible = 0;
    let mut removed = 0;
    let mut checked = false;

    loop {
        let mut to_remove = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r as usize][c as usize] != '@' {
                    continue;
                }
                let mut neighbors = 0;
                for dr in -1..=1_i32 {
                    for dc in -1..=1_i32 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let (nr, nc) = (r + dr, c + dc);
                        if 0 <= nr && nr < rows && 0 <= nc && nc < cols {
                            if grid[nr as usize][nc as usize] == '@' {
                                neighbors += 1;
                            }
                        }
                    }
                }
                if neighbors < 4 {
                    if !checked {
                        accessible += 1;
                    }
                    to_remove.push((r, c));
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }
        removed += to_remove.len();
        for (r, c) in to_remove {
            grid[r as usize][c as usize] = '.';
        }
        checked = true;
    }

    println!("Part 1: {}", accessible);
    println!("Part 2: {}", removed);
}
