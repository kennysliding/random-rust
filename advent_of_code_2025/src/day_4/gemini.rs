pub struct SolutionDay4Gemini;
use std::fs;

impl SolutionDay4Gemini {
    fn read_input(filename: &str) -> Vec<Vec<char>> {
        let input = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return Vec::new();
            }
        };
        input.lines().map(|s| s.chars().collect()).collect()
    }

    fn count_neighbors(grid: &Vec<Vec<char>>, r: i32, c: i32, rows: i32, cols: i32) -> i32 {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut count = 0;
        for &(dr, dc) in &offsets {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn solve_1(filename: &str) -> i32 {
        let grid = Self::read_input(filename);
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut accessible_count = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r as usize][c as usize] == '@' {
                    if Self::count_neighbors(&grid, r, c, rows, cols) < 4 {
                        accessible_count += 1;
                    }
                }
            }
        }

        println!("Part 1: {}", accessible_count);
        accessible_count
    }

    pub fn solve_2(filename: &str) -> i32 {
        let mut grid = Self::read_input(filename);
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut total_removed = 0;

        // Keep simulating rounds of removal until no more rolls can be removed
        loop {
            let mut to_remove = Vec::new();

            for r in 0..rows {
                for c in 0..cols {
                    // Check if current roll exists and is accessible
                    if grid[r as usize][c as usize] == '@' {
                        if Self::count_neighbors(&grid, r, c, rows, cols) < 4 {
                            to_remove.push((r, c));
                        }
                    }
                }
            }

            if to_remove.is_empty() {
                break;
            }

            total_removed += to_remove.len() as i32;

            // Remove the identified rolls from the grid so they don't count as neighbors in the next pass
            for (r, c) in to_remove {
                grid[r as usize][c as usize] = '.';
            }
        }

        println!("Part 2: {}", total_removed);
        total_removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay4Gemini::solve_1("src/day_4/input_test.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay4Gemini::solve_2("src/day_4/input_test.txt");
        assert_eq!(result, 43);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay4Gemini::solve_1("src/day_4/input.txt");
        assert_eq!(result, 1533);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay4Gemini::solve_2("src/day_4/input.txt");
        assert_eq!(result, 9206);
    }
}
