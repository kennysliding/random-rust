pub struct SolutionDay3;
use std::{cmp::max, fs};

impl SolutionDay3 {
    fn read_input(filename: &str) -> Vec<String> {
        let input = fs::read_to_string(filename).unwrap();
        input.lines().map(|s| s.to_string()).collect()
    }

    pub fn solve_1(filename: &str) -> u32 {
        let input = Self::read_input(filename);

        let mut joltages = Vec::new();
        for line in input {
            let sequence = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            // loop from the back and find the largest two digits
            let mut first_digit = sequence[sequence.len() - 2];
            let mut second_digit = sequence[sequence.len() - 1];
            let mut second_digit_candidate = second_digit;

            for &c in sequence.iter().rev().skip(2) {
                if c >= first_digit {
                    second_digit_candidate = max(second_digit_candidate, first_digit);
                    second_digit = max(second_digit_candidate, second_digit);
                    first_digit = c;
                    continue;
                }

                if c >= second_digit_candidate {
                    second_digit_candidate = c;
                }
            }

            let joltage = first_digit * 10 + second_digit;
            joltages.push(joltage);
        }

        return joltages.iter().sum();
    }

    pub fn solve_2(filename: &str) -> u64 {
        let input = Self::read_input(filename);

        let mut joltages: Vec<u64> = Vec::new();
        for line in input {
            let sequence = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>();

            // the first digit is for adding a new digit to the front
            let mut digits: Vec<u64> = sequence
                .iter()
                .skip(sequence.len() - 12)
                .map(|x| *x)
                .collect();

            // start from the back and keep iterating the same logic
            for (i, &c) in sequence.iter().enumerate().rev().skip(12) {
                if c >= digits[0] {
                    digits.insert(0, c);
                    // shifting the digits
                    for i in 1..digits.len() {
                        if digits[i] > digits[i - 1] {
                            digits.remove(i - 1);
                            break;
                        }
                    }
                }
            }

            let joltage = digits.iter().take(12).fold(0, |acc, x| acc * 10 + x);
            joltages.push(joltage);
        }

        return joltages.iter().map(|x| *x).sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay3::solve_1("src/day_3/input_test.txt");
        assert_eq!(result, 357);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay3::solve_2("src/day_3/input_test.txt");
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay3::solve_1("src/day_3/input.txt");
        assert_eq!(result, 17443);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay3::solve_2("src/day_3/input.txt");
        assert_eq!(result, 172167155440541);
    }
}
