pub struct SolutionDay2Gemini;
use std::fs;

impl SolutionDay2Gemini {
    /// Checks if a number is an "invalid ID".
    /// An invalid ID is a number formed by a sequence of digits repeated twice.
    /// Examples: 55, 123123, 446446.
    fn is_invalid_id_1(n: u64) -> bool {
        // IDs must have at least 2 digits to be a repeated sequence (e.g. 11).
        // Numbers < 10 cannot be split into two identical parts.
        if n < 10 {
            return false;
        }

        // 1. Get the number of digits.
        // ilog10() returns integer log10 (e.g., 99 -> 1, 100 -> 2). Adding 1 gives count.
        let digits = n.ilog10() + 1;

        // If the number of digits is odd, it cannot be two equal halves.
        if digits % 2 != 0 {
            return false;
        }

        // 2. Mathematical Check:
        // If n looks like "XYXY" (digits), mathematically n = XY * 10^k + XY
        // which factors to n = XY * (10^k + 1).
        // Therefore, n must be perfectly divisible by (10^k + 1).
        let half_digits = digits / 2;
        let divisor = 10u64.pow(half_digits) + 1;

        n % divisor == 0
    }

    /// Checks if a number is an "invalid ID" for Part 2.
    /// An ID is invalid if it is made only of some sequence of digits repeated at least twice.
    /// Examples: 121212 (12 repeated 3 times), 111 (1 repeated 3 times).
    fn is_invalid_id_2(n: u64) -> bool {
        if n < 10 {
            return false;
        }

        let digits = n.ilog10() + 1;

        // Try all possible repetition counts 'k' from 2 up to total digits.
        // 'k' represents how many times the pattern repeats (e.g., 121212 -> k=3).
        for k in 2..=digits {
            // The number of digits must be perfectly divisible by k.
            if digits % k == 0 {
                let s = digits / k; // 's' is the size of the repeating sequence (block size)

                // Construct the "repeater" number mathematically.
                // If sequence size s=2 and repeats k=3 times, repeater is 10101.
                // Formula: sum(10^(i*s)) for i in 0..k
                let mut repeater = 0u64;
                for i in 0..k {
                    repeater += 10u64.pow(i * s);
                }

                // If n is a valid repetition, it must be a multiple of the repeater.
                if n % repeater == 0 {
                    // We must also verify that the base sequence fits within 's' digits.
                    // n = base * repeater. We check if base < 10^s.
                    if (n / repeater) < 10u64.pow(s) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn solve_1(filename: &str) -> u64 {
        // Read the file content
        let content =
            fs::read_to_string(filename).expect("Should have been able to read the input file");

        let mut sum_invalid_ids: u64 = 0;

        // The input is a single line of comma-separated ranges (e.g., "11-22,95-115")
        // We trim whitespace and split by comma
        for range_str in content.trim().split(',') {
            // Each part is a range "min-max"
            if let Some((start_str, end_str)) = range_str.split_once('-') {
                let start: u64 = start_str.parse().expect("Invalid start of range");
                let end: u64 = end_str.parse().expect("Invalid end of range");

                // Check every ID in the range
                for id in start..=end {
                    if Self::is_invalid_id_1(id) {
                        sum_invalid_ids += id;
                    }
                }
            }
        }

        sum_invalid_ids
    }

    pub fn solve_2(filename: &str) -> u64 {
        let content =
            fs::read_to_string(filename).expect("Should have been able to read the input file");

        let mut sum_invalid_ids: u64 = 0;

        for range_str in content.trim().split(',') {
            if let Some((start_str, end_str)) = range_str.split_once('-') {
                let start: u64 = start_str.parse().expect("Invalid start of range");
                let end: u64 = end_str.parse().expect("Invalid end of range");

                for id in start..=end {
                    if Self::is_invalid_id_2(id) {
                        sum_invalid_ids += id;
                    }
                }
            }
        }

        sum_invalid_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay2Gemini::solve_1("src/day_2/input_test.txt");
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay2Gemini::solve_2("src/day_2/input_test.txt");
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay2Gemini::solve_1("src/day_2/input.txt");
        assert_eq!(result, 20223751480);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay2Gemini::solve_2("src/day_2/input.txt");
        assert_eq!(result, 30260171216);
    }
}
