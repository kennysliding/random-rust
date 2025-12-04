pub struct SolutionDay3Gemini;
use std::fs;

impl SolutionDay3Gemini {
    pub fn solve_1(filename: &str) -> i32 {
        // Read the file content
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return 0;
            }
        };

        let mut total_output_joltage = 0;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Convert the line into a vector of digits
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            // We need at least 2 batteries to turn on
            if digits.len() < 2 {
                continue;
            }

            let mut max_bank_joltage = 0;

            // Iterate through all pairs to find the maximum possible number
            // We need to pick digit at index i and digit at index j where i < j
            // The number formed is digits[i] * 10 + digits[j]
            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let joltage = digits[i] * 10 + digits[j];
                    if joltage > max_bank_joltage {
                        max_bank_joltage = joltage;
                    }
                }
            }

            total_output_joltage += max_bank_joltage as i32;
        }

        println!("{}", total_output_joltage);
        total_output_joltage
    }

    pub fn solve_2(filename: &str) -> i64 {
        // Read the file content
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return 0;
            }
        };

        let mut total_output_joltage: i64 = 0;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Convert the line into a vector of digits
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            // Part 2: We need exactly 12 batteries
            if digits.len() < 12 {
                continue;
            }

            // We want to pick a subsequence of length 12 that forms the largest number.
            // This is equivalent to removing (len - 12) digits to maximize the result.
            let to_remove = digits.len() - 12;
            let mut drops_remaining = to_remove;
            let mut stack: Vec<u32> = Vec::new();

            for &digit in &digits {
                // While we can still drop digits, and the current digit is larger
                // than the last one we picked, discard the smaller one (greedy approach).
                while drops_remaining > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
                    stack.pop();
                    drops_remaining -= 1;
                }
                stack.push(digit);
            }

            // If we haven't dropped enough (e.g., numbers were already in descending order),
            // truncate the end to ensure we only have 12 digits.
            stack.truncate(12);

            // Construct the 12-digit number from the stack
            let mut max_bank_joltage: i64 = 0;
            for d in stack {
                max_bank_joltage = max_bank_joltage * 10 + d as i64;
            }

            total_output_joltage += max_bank_joltage;
        }

        println!("{}", total_output_joltage);
        total_output_joltage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay3Gemini::solve_1("src/day_3/input_test.txt");
        assert_eq!(result, 357);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay3Gemini::solve_2("src/day_3/input_test.txt");
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay3Gemini::solve_1("src/day_3/input.txt");
        assert_eq!(result, 17443);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay3Gemini::solve_2("src/day_3/input.txt");
        assert_eq!(result, 172167155440541);
    }
}
