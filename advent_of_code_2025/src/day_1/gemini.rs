pub struct SolutionDay1Gemini;
use std::fs;

impl SolutionDay1Gemini {
    pub fn solve_1(filename: &str) -> i32 {
        // Read the file content
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return 0;
            }
        };

        // The dial has numbers 0 through 99 (100 total positions)
        // It starts pointing at 50.
        let mut current_pos: i32 = 50;
        let mut zero_visits: i32 = 0;
        let dial_size: i32 = 100;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse the direction (first character) and the amount (rest of the string)
            let direction = line.chars().next().unwrap();

            // Parse the numeric part safely
            let amount: i32 = match line[1..].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Skipping invalid line: {}", line);
                    continue;
                }
            };

            // Calculate new position based on direction
            match direction {
                'R' => {
                    // Moving Right (increasing numbers)
                    // (Current + Amount) % 100 handles the wrapping past 99 to 0
                    current_pos = (current_pos + amount) % dial_size;
                }
                'L' => {
                    // Moving Left (decreasing numbers)
                    // Rust's % operator calculates the remainder, so it can return negative numbers.
                    // To handle wrapping (e.g., 0 -> 99), we ensure the result is positive.
                    // Formula: ((Current - Amount) % 100 + 100) % 100
                    current_pos = (current_pos - amount) % dial_size;
                    if current_pos < 0 {
                        current_pos += dial_size;
                    }
                }
                _ => {
                    eprintln!("Unknown direction: {}", direction);
                    continue;
                }
            }

            // Check if the dial landed on 0
            if current_pos == 0 {
                zero_visits += 1;
            }
        }

        // Print to stdout as requested in the text prompt description
        println!("{}", zero_visits);

        // Return the value as requested by the code structure signature
        zero_visits
    }

    pub fn solve_2(filename: &str) -> i32 {
        // Read the file content
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return 0;
            }
        };

        // The dial has numbers 0 through 99 (100 total positions)
        // It starts pointing at 50.
        let mut current_pos: i32 = 50;
        let mut zero_visits: i32 = 0;
        let dial_size: i32 = 100;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse the direction (first character) and the amount (rest of the string)
            let direction = line.chars().next().unwrap();

            // Parse the numeric part safely
            let amount: i32 = match line[1..].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Skipping invalid line: {}", line);
                    continue;
                }
            };

            // Part 2 Logic: Count every time the dial hits 0.

            // 1. Calculate full revolutions.
            // A full revolution of 100 ticks guarantees exactly one '0' visit,
            // regardless of the starting position.
            zero_visits += amount / dial_size;

            // 2. Simulate the remaining ticks.
            let remaining_steps = amount % dial_size;

            for _ in 0..remaining_steps {
                match direction {
                    'R' => {
                        // Moving Right (increasing numbers)
                        current_pos = (current_pos + 1) % dial_size;
                    }
                    'L' => {
                        // Moving Left (decreasing numbers)
                        // Adding dial_size ensures the result is positive before modulo
                        current_pos = (current_pos - 1 + dial_size) % dial_size;
                    }
                    _ => {
                        eprintln!("Unknown direction: {}", direction);
                        break;
                    }
                }

                // Check if the dial landed on 0 during the tick
                if current_pos == 0 {
                    zero_visits += 1;
                }
            }
        }

        // Print to stdout as requested in the text prompt description
        println!("{}", zero_visits);

        // Return the value as requested by the code structure signature
        zero_visits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        assert_eq!(SolutionDay1Gemini::solve_1("src/day_1/input_test.txt"), 3);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(SolutionDay1Gemini::solve_2("src/day_1/input_test.txt"), 6);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        assert_eq!(SolutionDay1Gemini::solve_1("src/day_1/input.txt"), 962);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        assert_eq!(SolutionDay1Gemini::solve_2("src/day_1/input.txt"), 5782);
    }
}
