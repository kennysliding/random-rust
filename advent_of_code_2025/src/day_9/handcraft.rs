pub struct SolutionDay9;
use std::collections::HashSet;
use std::{
    cmp::{max, min},
    fs,
};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn area_with(&self, other: &Coordinate) -> u64 {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;
        width as u64 * height as u64
    }
}

#[derive(Debug)]
struct VerticalEdge {
    x: usize,
    y_min: usize,
    y_max: usize,
}

impl SolutionDay9 {
    fn read_input(filename: &str) -> Vec<Coordinate> {
        let input = fs::read_to_string(filename).unwrap();
        input
            .lines()
            .map(|line| {
                let parts = line
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                Coordinate::new(parts[0], parts[1])
            })
            .collect()
    }

    pub fn solve_1(filename: &str) -> u64 {
        let coordinates = Self::read_input(filename);

        let mut max_area = 0;
        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                let area = coordinates[i].area_with(&coordinates[j]);
                max_area = max(max_area, area);
            }
        }
        max_area
    }

    fn is_inside_by_raycasting(x: usize, y: usize, vertical_edges: &[VerticalEdge]) -> bool {
        let mut crossings = 0;

        for edge in vertical_edges {
            if edge.x < x && edge.y_min < y && y <= edge.y_max {
                crossings += 1;
            }
        }

        crossings % 2 == 1
    }

    pub fn solve_2(filename: &str) -> u64 {
        // brute force approach
        let coordinates = Self::read_input(filename);

        let mut edge_nodes: HashSet<Coordinate> = HashSet::new();
        // precompute vertical edges for raycasting
        let mut vertical_edges: Vec<VerticalEdge> = Vec::new();

        // handle each pair of coordinates
        for i in 0..coordinates.len() {
            let coordinate_a = coordinates[i];
            let coordinate_b = coordinates[(i + 1) % coordinates.len()];

            let start_x = min(coordinate_a.x, coordinate_b.x);
            let end_x = max(coordinate_a.x, coordinate_b.x);
            let start_y = min(coordinate_a.y, coordinate_b.y);
            let end_y = max(coordinate_a.y, coordinate_b.y);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    edge_nodes.insert(Coordinate::new(x, y));
                }
            }

            if coordinate_a.x == coordinate_b.x {
                vertical_edges.push(VerticalEdge {
                    x: coordinate_a.x,
                    y_min: start_y,
                    y_max: end_y,
                });
            }
        }

        let mut max_area = 0;
        let mut inside_coordinates: HashSet<Coordinate> = HashSet::new();
        let mut outside_coordinates: HashSet<Coordinate> = HashSet::new();

        let unique_ys: Vec<usize> = coordinates
            .iter()
            .map(|c| c.y)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                let coordinate_a = coordinates[i];
                let coordinate_b = coordinates[j];

                let max_x = max(coordinate_a.x, coordinate_b.x);
                let max_y = max(coordinate_a.y, coordinate_b.y);
                let min_x = min(coordinate_a.x, coordinate_b.x);
                let min_y = min(coordinate_a.y, coordinate_b.y);

                // check if all the coordinates within the bounding box are inside
                let mut all_inside = true;

                'outer: for &y in &unique_ys {
                    if y < min_y || y > max_y {
                        continue;
                    }
                    for x in [min_x, max_x] {
                        if edge_nodes.contains(&Coordinate::new(x, y)) {
                            continue;
                        }

                        if inside_coordinates.contains(&Coordinate::new(x, y)) {
                            continue;
                        }

                        if outside_coordinates.contains(&Coordinate::new(x, y)) {
                            all_inside = false;
                            break 'outer;
                        }

                        if Self::is_inside_by_raycasting(x, y, &vertical_edges) {
                            inside_coordinates.insert(Coordinate::new(x, y));
                            continue;
                        }

                        outside_coordinates.insert(Coordinate::new(x, y));
                        all_inside = false;
                        break 'outer;
                    }
                }

                if all_inside {
                    let area = coordinates[i].area_with(&coordinates[j]);
                    max_area = max(max_area, area);
                }
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay9::solve_1("src/day_9/input_test.txt");
        assert_eq!(result, 50);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay9::solve_2("src/day_9/input_test.txt");
        assert_eq!(result, 24);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay9::solve_1("src/day_9/input.txt");
        assert_eq!(result, 4748769124);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay9::solve_2("src/day_9/input.txt");
        assert_eq!(result, 1525991432);
    }
}
