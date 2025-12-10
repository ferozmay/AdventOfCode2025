use std::{env, fs};

// INCOMPLETE :(

// My approach:
// Parse the junction box coordinates
// Compute distances between all pairs of points
// Store these in a list along with each point's index
// Sort this list by distance
// For the 1000 shortest pairs in this list, implement UnionFind
//  - If two boxes are already in the same circuit: do nothing
//  - If different circuits: then merge them
// Find the 3 largest circuits and multiply their size

fn euclidean_dist_squared(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            // Merge smaller into larger
            if self.size[ra] < self.size[rb] {
                self.parent[ra] = rb;
                self.size[rb] += self.size[ra];
            } else {
                self.parent[rb] = ra;
                self.size[ra] += self.size[rb];
            }
        }
    }
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

    let n = points.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n - 1 {
        for j in (i + 1)..n {
            let dist = euclidean_dist_squared(points[i], points[j]);
            edges.push((dist, i, j));
        }
    }
    edges.sort();
    // edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut uf = UnionFind::new(n);
    for (_, i, j) in edges.iter().take(1000) {
        uf.union(*i, *j);
    }

    let mut sizes: Vec<usize> = (0..n)
        .filter(|&i| uf.find(i) == i) // Only roots
        .map(|i| uf.size[i])
        .collect();
    sizes.sort_by(|a, b| b.cmp(a)); // Descending

    println!("{}", sizes[0] * sizes[1] * sizes[2]);
}
