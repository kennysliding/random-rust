pub struct SolutionDay8Gemini;
use std::fs;

// A simple implementation of the Disjoint Set Union (Union-Find) data structure
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize, // Track number of disjoint sets
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    // Find the representative (root) of the set containing 'i' with path compression
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    // Unite the sets containing 'i' and 'j'. Returns true if they were different sets.
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            // Merge smaller set into larger set
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.count -= 1; // One less disjoint set
            true
        } else {
            false
        }
    }
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl SolutionDay8Gemini {
    fn read_input(filename: &str) -> Vec<Point> {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return Vec::new();
            }
        };

        content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.trim().split(',').collect();
                if parts.len() == 3 {
                    Some(Point {
                        x: parts[0].parse().unwrap_or(0),
                        y: parts[1].parse().unwrap_or(0),
                        z: parts[2].parse().unwrap_or(0),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    // Helper to generate sorted edges
    fn get_sorted_edges(points: &Vec<Point>) -> Vec<(u64, usize, usize)> {
        let n = points.len();
        let mut edges: Vec<(u64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);

        for i in 0..n {
            for j in (i + 1)..n {
                let p1 = &points[i];
                let p2 = &points[j];
                let dist_sq = (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2);
                edges.push((dist_sq as u64, i, j));
            }
        }

        // Sort pairs by distance (shortest first)
        edges.sort_unstable_by_key(|k| k.0);
        edges
    }

    pub fn solve_1(filename: &str, connecting_pairs: usize) -> u64 {
        let points = Self::read_input(filename);
        let n = points.len();
        if n == 0 {
            return 0;
        }

        let edges = Self::get_sorted_edges(&points);

        // Process the top 1000 closest pairs
        let mut uf = UnionFind::new(n);
        let limit = std::cmp::min(connecting_pairs, edges.len());

        for i in 0..limit {
            let (_, u, v) = edges[i];
            uf.union(u, v);
        }

        // Collect sizes of all distinct circuits (roots)
        let mut sizes: Vec<u64> = Vec::new();
        let mut seen_roots = vec![false; n];

        for i in 0..n {
            let root = uf.find(i);
            if !seen_roots[root] {
                sizes.push(uf.size[root] as u64);
                seen_roots[root] = true;
            }
        }

        sizes.sort_unstable_by(|a, b| b.cmp(a)); // Descending sort

        let result = sizes.iter().take(3).product();
        println!("Part 1 Result: {}", result);
        result
    }

    pub fn solve_2(filename: &str) -> u64 {
        let points = Self::read_input(filename);
        let n = points.len();
        if n == 0 {
            return 0;
        }

        let edges = Self::get_sorted_edges(&points);
        let mut uf = UnionFind::new(n);

        // Iterate through edges until only 1 set remains
        for (_, u, v) in edges {
            let merged = uf.union(u, v);

            if merged && uf.count == 1 {
                // This was the connection that united the last two components
                let result = points[u].x * points[v].x;
                println!("Part 2 Result: {}", result);
                return result as u64;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay8Gemini::solve_1("src/day_8/input_test.txt", 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay8Gemini::solve_2("src/day_8/input_test.txt");
        assert_eq!(result, 25272);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay8Gemini::solve_1("src/day_8/input.txt", 1000);
        assert_eq!(result, 68112);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay8Gemini::solve_2("src/day_8/input.txt");
        assert_eq!(result, 44543856);
    }
}
