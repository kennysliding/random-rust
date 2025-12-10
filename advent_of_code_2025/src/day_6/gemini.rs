pub struct SolutionDay6Gemini;
use std::{cmp::max, fs};

impl SolutionDay6Gemini {
    // Helper to read file and produce a padded character grid
    fn read_padded_grid(filename: &str) -> Vec<Vec<char>> {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return Vec::new();
            }
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        if lines.is_empty() {
            return Vec::new();
        }

        let max_len = lines.iter().map(|s| s.len()).max().unwrap_or(0);

        lines
            .iter()
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                while chars.len() < max_len {
                    chars.push(' ');
                }
                chars
            })
            .collect()
    }

    pub fn solve_1(filename: &str) -> u64 {
        let grid = Self::read_padded_grid(filename);
        if grid.is_empty() {
            return 0;
        }

        let cols = grid[0].len();
        let mut grand_total: u64 = 0;
        let mut col = 0;

        while col < cols {
            if Self::is_col_empty(&grid, col) {
                col += 1;
                continue;
            }

            let start_col = col;
            while col < cols && !Self::is_col_empty(&grid, col) {
                col += 1;
            }
            let end_col = col;

            grand_total += Self::process_block_part1(&grid, start_col, end_col);
        }

        println!("Part 1: {}", grand_total);
        grand_total
    }

    pub fn solve_2(filename: &str) -> u64 {
        let grid = Self::read_padded_grid(filename);
        if grid.is_empty() {
            return 0;
        }

        let cols = grid[0].len();
        let mut grand_total: u64 = 0;
        let mut col = 0;

        while col < cols {
            if Self::is_col_empty(&grid, col) {
                col += 1;
                continue;
            }

            let start_col = col;
            while col < cols && !Self::is_col_empty(&grid, col) {
                col += 1;
            }
            let end_col = col;

            grand_total += Self::process_block_part2(&grid, start_col, end_col);
        }

        println!("Part 2: {}", grand_total);
        grand_total
    }

    fn is_col_empty(grid: &Vec<Vec<char>>, col: usize) -> bool {
        for row in grid {
            if col < row.len() && !row[col].is_whitespace() {
                return false;
            }
        }
        true
    }

    // Part 1 Parsing: Horizontal numbers (rows)
    fn process_block_part1(grid: &Vec<Vec<char>>, start: usize, end: usize) -> u64 {
        let mut numbers: Vec<u64> = Vec::new();
        let mut operator = None;

        for row in grid {
            if start >= row.len() {
                continue;
            }
            let slice = &row[start..std::cmp::min(end, row.len())];

            let s: String = slice.iter().collect();
            let trimmed = s.trim();

            if trimmed.is_empty() {
                continue;
            }

            if trimmed == "+" {
                operator = Some('+');
            } else if trimmed == "*" {
                operator = Some('*');
            } else if let Ok(n) = trimmed.parse::<u64>() {
                numbers.push(n);
            }
        }

        match operator {
            Some('+') => numbers.iter().sum(),
            Some('*') => numbers.iter().product(),
            _ => 0,
        }
    }

    // Part 2 Parsing: Vertical numbers (columns)
    fn process_block_part2(grid: &Vec<Vec<char>>, start: usize, end: usize) -> u64 {
        let mut numbers: Vec<u64> = Vec::new();
        let mut operator = None;

        // Iterate through columns in the block to extract numbers vertically
        for col in start..end {
            let mut num_str = String::new();

            for row in grid {
                let c = row[col];
                if c.is_ascii_digit() {
                    num_str.push(c);
                } else if c == '+' {
                    operator = Some('+');
                } else if c == '*' {
                    operator = Some('*');
                }
            }

            if !num_str.is_empty() {
                if let Ok(n) = num_str.parse::<u64>() {
                    numbers.push(n);
                }
            }
        }

        match operator {
            Some('+') => numbers.iter().sum(),
            Some('*') => numbers.iter().product(),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay6Gemini::solve_1("src/day_6/input_test.txt");
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay6Gemini::solve_2("src/day_6/input_test.txt");
        assert_eq!(result, 3263827);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay6Gemini::solve_1("src/day_6/input.txt");
        assert_eq!(result, 4076006202939);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay6Gemini::solve_2("src/day_6/input.txt");
        assert_eq!(result, 7903168391557);
    }
}
