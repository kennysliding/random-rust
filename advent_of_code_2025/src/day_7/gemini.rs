pub struct SolutionDay7Gemini;
use std::collections::HashSet;
use std::fs;

impl SolutionDay7Gemini {
    fn read_grid(filename: &str) -> Vec<Vec<char>> {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return Vec::new();
            }
        };
        content.lines().map(|line| line.chars().collect()).collect()
    }

    fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for (r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    return Some((r, c));
                }
            }
        }
        None
    }

    pub fn solve_1(filename: &str) -> u64 {
        let grid = Self::read_grid(filename);
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut split_count = 0;

        let (start_row, start_col) = match Self::find_start(&grid) {
            Some(pos) => pos,
            None => return 0,
        };

        // Track unique active columns (Merging behavior)
        let mut current_beams = HashSet::new();
        current_beams.insert(start_col);

        for r in start_row..rows {
            let mut next_beams = HashSet::new();

            if current_beams.is_empty() {
                break;
            }

            for &c in &current_beams {
                if c >= cols {
                    continue;
                }
                match grid[r][c] {
                    '^' => {
                        split_count += 1;
                        if c > 0 {
                            next_beams.insert(c - 1);
                        }
                        if c + 1 < cols {
                            next_beams.insert(c + 1);
                        }
                    }
                    _ => {
                        next_beams.insert(c);
                    }
                }
            }
            current_beams = next_beams;
        }

        println!("Part 1: {}", split_count);
        split_count
    }

    pub fn solve_2(filename: &str) -> u64 {
        let grid = Self::read_grid(filename);
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();

        let (start_row, start_col) = match Self::find_start(&grid) {
            Some(pos) => pos,
            None => return 0,
        };

        // Track count of timelines at each column (Summing behavior)
        let mut current_counts = vec![0u64; cols];
        current_counts[start_col] = 1;

        for r in start_row..rows {
            let mut next_counts = vec![0u64; cols];
            let mut active = false;

            for c in 0..cols {
                let count = current_counts[c];
                if count == 0 {
                    continue;
                }
                active = true;

                match grid[r][c] {
                    '^' => {
                        // Split: timelines diverge to left and right
                        // The count of timelines splits into both directions
                        if c > 0 {
                            next_counts[c - 1] += count;
                        }
                        if c + 1 < cols {
                            next_counts[c + 1] += count;
                        }
                    }
                    _ => {
                        // Pass through: count moves down
                        next_counts[c] += count;
                    }
                }
            }

            if !active {
                break;
            }
            current_counts = next_counts;
        }

        let total_timelines: u64 = current_counts.iter().sum();
        println!("Part 2: {}", total_timelines);
        total_timelines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay7Gemini::solve_1("src/day_7/input_test.txt");
        assert_eq!(result, 21);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay7Gemini::solve_2("src/day_7/input_test.txt");
        assert_eq!(result, 40);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay7Gemini::solve_1("src/day_7/input.txt");
        assert_eq!(result, 1516);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay7Gemini::solve_2("src/day_7/input.txt");
        assert_eq!(result, 1393669447690);
    }
}
