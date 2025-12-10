pub struct SolutionDay8;
use std::{cmp::max, collections::HashMap, fs, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coordinate {
    x: u64,
    y: u64,
    z: u64,
}

impl Coordinate {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &Coordinate) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);

        // (x^2 + y^2 + z^2)^(1/2)
        (dx.pow(2) + dy.pow(2) + dz.pow(2)).isqrt()
    }
}

impl SolutionDay8 {
    fn read_input(filename: &str) -> Vec<Coordinate> {
        let input = fs::read_to_string(filename).unwrap();
        input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .map(|coordinates| Coordinate::new(coordinates[0], coordinates[1], coordinates[2]))
            .collect()
    }

    fn union(
        circuits: &mut HashMap<Coordinate, Coordinate>,
        coordinate_a: &Coordinate,
        coordinate_b: &Coordinate,
    ) {
        // move all the roots from A to B
        let a_root = Self::find_parent(circuits, coordinate_a);
        let b_root = Self::find_parent(circuits, coordinate_b);

        for (coordinate, _) in circuits.clone().into_iter() {
            if Self::find_parent(circuits, &coordinate) == a_root {
                circuits.insert(coordinate, b_root);
            }
        }
    }

    fn find_parent(
        circuits: &HashMap<Coordinate, Coordinate>,
        coordinate: &Coordinate,
    ) -> Coordinate {
        let parent = *circuits.get(coordinate).unwrap();
        if *coordinate == parent {
            // root
            return parent;
        }

        return Self::find_parent(circuits, &parent);
    }

    pub fn solve_1(filename: &str, connecting_pairs: usize) -> u128 {
        let coordinates = Self::read_input(filename);
        let mut distances: Vec<(Coordinate, Coordinate, u64)> = vec![];

        // calculate the distances between all pairs of coordinates
        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                distances.push((
                    coordinates[i].clone(),
                    coordinates[j].clone(),
                    coordinates[i].distance_to(&coordinates[j]),
                ));
            }
        }

        // sort the distances by distance
        distances.sort_by_key(|(_, _, distance)| *distance);

        // connect the 1000 closest pairs using union-find
        // this > parent
        let mut circuits: HashMap<Coordinate, Coordinate> = HashMap::new();

        for (coordinate_a, coordinate_b, _distance) in distances.iter().take(connecting_pairs) {
            if !circuits.contains_key(coordinate_a) && !circuits.contains_key(coordinate_b) {
                // create a circuit
                circuits.insert(*coordinate_a, *coordinate_a); // root
                circuits.insert(*coordinate_b, *coordinate_a);
                continue;
            }

            // case when A has a parent but B doesn't > add B to A's circuit
            if circuits.contains_key(coordinate_a) && !circuits.contains_key(coordinate_b) {
                circuits.insert(*coordinate_b, *coordinate_a);
                continue;
            }

            // case when B has a parent but A doesn't > add A to B's circuit
            if circuits.contains_key(coordinate_b) && !circuits.contains_key(coordinate_a) {
                circuits.insert(*coordinate_a, *coordinate_b);
                continue;
            }

            // case when both A and B has a parent
            // different parent == different circuit > connect them
            if Self::find_parent(&mut circuits, coordinate_a)
                != Self::find_parent(&mut circuits, coordinate_b)
            {
                Self::union(&mut circuits, coordinate_a, coordinate_b);
            }

            // case when both A and B has the same parent
            // same parent == same circuit > do nothing
        }

        let mut circuit_sizes: HashMap<Coordinate, u64> = HashMap::new();
        for (coordinate, _) in circuits.clone().into_iter() {
            let root = Self::find_parent(&circuits, &coordinate);

            // add the size of the circuit
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }

        // find the 3 top circuits with max sizes
        let mut top_sizes = circuit_sizes
            .into_values()
            .map(|size| size as u128)
            .collect::<Vec<u128>>();
        top_sizes.sort_by_key(|size| *size);
        top_sizes.reverse();
        top_sizes.iter().take(3).product()
    }

    pub fn solve_2(filename: &str) -> u64 {
        let coordinates = Self::read_input(filename);
        let mut distances: Vec<(Coordinate, Coordinate, u64)> = vec![];

        // calculate the distances between all pairs of coordinates
        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                distances.push((
                    coordinates[i].clone(),
                    coordinates[j].clone(),
                    coordinates[i].distance_to(&coordinates[j]),
                ));
            }
        }

        // sort the distances by distance
        distances.sort_by_key(|(_, _, distance)| *distance);

        let mut circuit_count = coordinates.len();
        let mut circuits: HashMap<Coordinate, Coordinate> = HashMap::new();

        for (coordinate_a, coordinate_b, _distance) in distances.iter() {
            if !circuits.contains_key(coordinate_a) && !circuits.contains_key(coordinate_b) {
                // create a circuit
                circuits.insert(*coordinate_a, *coordinate_a); // root
                circuits.insert(*coordinate_b, *coordinate_a);
                // since we are connecting two boxes, we decrement the circuit count
                circuit_count -= 1;
                if circuit_count == 1 {
                    // return the product of the x coordinates of the last two boxes
                    return coordinate_a.x * coordinate_b.x;
                }
                continue;
            }

            // case when A has a parent but B doesn't > add B to A's circuit
            if circuits.contains_key(coordinate_a) && !circuits.contains_key(coordinate_b) {
                circuits.insert(*coordinate_b, *coordinate_a);
                // since we are connecting two boxes, we decrement the circuit count
                circuit_count -= 1;
                if circuit_count == 1 {
                    // return the product of the x coordinates of the last two boxes
                    return coordinate_a.x * coordinate_b.x;
                }
                continue;
            }

            // case when B has a parent but A doesn't > add A to B's circuit
            if circuits.contains_key(coordinate_b) && !circuits.contains_key(coordinate_a) {
                circuits.insert(*coordinate_a, *coordinate_b);
                // since we are connecting two boxes, we decrement the circuit count
                circuit_count -= 1;
                if circuit_count == 1 {
                    // return the product of the x coordinates of the last two boxes
                    return coordinate_a.x * coordinate_b.x;
                }
                continue;
            }

            // case when both A and B has a parent
            // different parent == different circuit > connect them
            if Self::find_parent(&mut circuits, coordinate_a)
                != Self::find_parent(&mut circuits, coordinate_b)
            {
                Self::union(&mut circuits, coordinate_a, coordinate_b);
                // since we are connecting two boxes, we decrement the circuit count
                circuit_count -= 1;
                if circuit_count == 1 {
                    // return the product of the x coordinates of the last two boxes
                    return coordinate_a.x * coordinate_b.x;
                }
            }
        }

        panic!("No solution found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay8::solve_1("src/day_8/input_test.txt", 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay8::solve_2("src/day_8/input_test.txt");
        assert_eq!(result, 25272);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay8::solve_1("src/day_8/input.txt", 1000);
        assert_eq!(result, 68112);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay8::solve_2("src/day_8/input.txt");
        assert_eq!(result, 44543856);
    }
}
