pub struct SolutionDay5Gemini;
use std::{cmp::max, fs};

impl SolutionDay5Gemini {
    // Helper to read and parse the file into ranges and IDs
    fn read_and_parse(filename: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return (Vec::new(), Vec::new());
            }
        };

        let mut ranges = Vec::new();
        let mut ids = Vec::new();
        let mut parsing_ranges = true;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                parsing_ranges = false;
                continue;
            }

            if parsing_ranges {
                if let Some((start_str, end_str)) = line.split_once('-') {
                    let start: u64 = start_str.parse().unwrap_or(0);
                    let end: u64 = end_str.parse().unwrap_or(0);
                    ranges.push((start, end));
                }
            } else {
                if let Ok(id) = line.parse::<u64>() {
                    ids.push(id);
                }
            }
        }
        (ranges, ids)
    }

    pub fn solve_1(filename: &str) -> usize {
        let (ranges, ids) = Self::read_and_parse(filename);
        let mut fresh_count = 0;

        for id in ids {
            // Check if this ID falls into ANY of the ranges
            for &(start, end) in &ranges {
                if id >= start && id <= end {
                    fresh_count += 1;
                    break; // It's fresh, no need to check other ranges
                }
            }
        }

        println!("Part 1: {}", fresh_count);
        fresh_count
    }

    pub fn solve_2(filename: &str) -> u64 {
        let (mut ranges, _) = Self::read_and_parse(filename);

        if ranges.is_empty() {
            return 0;
        }

        // Sort ranges by start point to enable linear merging
        ranges.sort_by_key(|k| k.0);

        let mut total_fresh_ids = 0;

        // Start with the first range
        let mut current_start = ranges[0].0;
        let mut current_end = ranges[0].1;

        for &(next_start, next_end) in ranges.iter().skip(1) {
            // If the next range starts inside or immediately after the current range, merge them.
            // checking `+ 1` handles adjacent integers (e.g., 3-5 and 6-8 should merge)
            if next_start <= current_end + 1 {
                current_end = max(current_end, next_end);
            } else {
                // No overlap: add the count of the current range...
                total_fresh_ids += current_end - current_start + 1;

                // ...and move to the next range
                current_start = next_start;
                current_end = next_end;
            }
        }

        // Add the final range
        total_fresh_ids += current_end - current_start + 1;

        println!("Part 2: {}", total_fresh_ids);
        total_fresh_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay5Gemini::solve_1("src/day_5/input_test.txt");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay5Gemini::solve_2("src/day_5/input_test.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay5Gemini::solve_1("src/day_5/input.txt");
        assert_eq!(result, 520);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay5Gemini::solve_2("src/day_5/input.txt");
        assert_eq!(result, 347338785050515);
    }
}
