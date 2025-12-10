pub struct SolutionDay7;
use std::collections::{HashMap, HashSet};
use std::fs;

impl SolutionDay7 {
    fn read_input(filename: &str) -> Vec<String> {
        let input = fs::read_to_string(filename).unwrap();
        input.lines().map(|line| line.to_string()).collect()
    }

    pub fn solve_1(filename: &str) -> u32 {
        let input = Self::read_input(filename);

        // find the index of 'S;
        let s_index = input[0].chars().position(|c| c == 'S').unwrap();

        let mut beam_positions = HashSet::new();
        beam_positions.insert(s_index);
        let mut split_count = 0;

        for row in input.iter().skip(2).step_by(2) {
            let mut new_beam_positions = HashSet::new();

            for beam_position in beam_positions {
                match row.chars().nth(beam_position).unwrap() {
                    '^' => {
                        new_beam_positions.insert(beam_position - 1);
                        new_beam_positions.insert(beam_position + 1);
                        split_count += 1;
                    }
                    '.' => {
                        new_beam_positions.insert(beam_position);
                    }
                    _ => panic!(
                        "Invalid character: {}",
                        row.chars().nth(beam_position).unwrap()
                    ),
                }
            }

            beam_positions = new_beam_positions;
        }

        split_count
    }

    pub fn solve_2(filename: &str) -> u64 {
        let input = Self::read_input(filename);

        // find the index of 'S;
        let s_index = input[0].chars().position(|c| c == 'S').unwrap();

        let mut beam_path_count: HashMap<usize, u64> = HashMap::new();
        beam_path_count.insert(s_index, 1);

        for row in input.iter().skip(2).step_by(2) {
            let mut new_beam_path_count = HashMap::new();

            for (beam_position, path_count) in beam_path_count {
                match row.chars().nth(beam_position).unwrap() {
                    '^' => {
                        // if the beam position hasn't been reached yet
                        if !new_beam_path_count.contains_key(&(beam_position - 1)) {
                            new_beam_path_count.insert(beam_position - 1, path_count);
                        } else {
                            new_beam_path_count
                                .entry(beam_position - 1)
                                .and_modify(|x| *x += path_count);
                        }

                        if !new_beam_path_count.contains_key(&(beam_position + 1)) {
                            new_beam_path_count.insert(beam_position + 1, path_count);
                        } else {
                            new_beam_path_count
                                .entry(beam_position + 1)
                                .and_modify(|x| *x += path_count);
                        }
                    }
                    '.' => {
                        if !new_beam_path_count.contains_key(&beam_position) {
                            new_beam_path_count.insert(beam_position, path_count);
                        } else {
                            new_beam_path_count
                                .entry(beam_position)
                                .and_modify(|x| *x += path_count);
                        }
                    }
                    _ => panic!(
                        "Invalid character: {}",
                        row.chars().nth(beam_position).unwrap()
                    ),
                }
            }

            beam_path_count = new_beam_path_count;
        }

        beam_path_count.values().into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay7::solve_1("src/day_7/input_test.txt");
        assert_eq!(result, 21);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay7::solve_2("src/day_7/input_test.txt");
        assert_eq!(result, 40);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay7::solve_1("src/day_7/input.txt");
        assert_eq!(result, 1516);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay7::solve_2("src/day_7/input.txt");
        assert_eq!(result, 1393669447690);
    }
}
