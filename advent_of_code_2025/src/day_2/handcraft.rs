pub struct SolutionDay2;
use std::fs;

impl SolutionDay2 {
    fn read_input(filename: &str) -> Vec<String> {
        let input = fs::read_to_string(filename).unwrap();
        input.split(',').map(|s| s.to_string()).collect()
    }

    pub fn is_invalid_1(id: u64) -> bool {
        let id_str = id.to_string();
        if id_str.len() % 2 != 0 {
            return false;
        }

        return id_str[..id_str.len() / 2] == id_str[id_str.len() / 2..];
    }

    pub fn solve_1(filename: &str) -> u64 {
        let mut invalid_ids: Vec<u64> = Vec::new();

        for range in Self::read_input(filename) {
            let parts = range.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();

            for id in start..=end {
                if Self::is_invalid_1(id) {
                    invalid_ids.push(id);
                }
            }
        }
        invalid_ids.iter().sum()
    }

    pub fn is_invalid_2(id: u64) -> bool {
        let id_str = id.to_string();
        for i in 1..=id_str.len() / 2 {
            let repeat_part = id_str[..i].to_string();

            let repeat_count = id_str.matches(&repeat_part).count();

            if repeat_count * i == id_str.len() {
                return true;
            }
        }
        false
    }

    pub fn is_invalid_2_optimized(id: u64) -> bool {
        let s = id.to_string(); // Only 1 allocation per number
        let n = s.len();

        // We only need to check lengths that divide the total length perfectly
        // e.g., if length is 6, checking a pattern of length 4 is useless (6 % 4 != 0)
        for len in 1..=n / 2 {
            if n % len == 0 {
                let pattern = &s[..len];
                let repetitions = n / len;

                // Check if repeating the pattern creates the original string
                if pattern.repeat(repetitions) == s {
                    return true;
                }
            }
        }
        false
    }

    pub fn solve_2(filename: &str) -> u64 {
        let mut invalid_ids: Vec<u64> = Vec::new();

        for range in Self::read_input(filename) {
            let parts = range.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();

            for id in start..=end {
                if Self::is_invalid_2_optimized(id) {
                    invalid_ids.push(id);
                }
            }
        }
        invalid_ids.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_1() {
        assert_eq!(SolutionDay2::is_invalid_1(123_123), true);
        assert_eq!(SolutionDay2::is_invalid_1(1231_234), false);
        assert_eq!(SolutionDay2::is_invalid_1(1231_1231), true);
        assert_eq!(SolutionDay2::is_invalid_1(12312_3456), false);
        assert_eq!(SolutionDay2::is_invalid_1(7777_7777), true);
        assert_eq!(SolutionDay2::is_invalid_1(0), false);
    }

    #[test]
    fn test_is_invalid_2() {
        // case for repeated one time
        assert_eq!(SolutionDay2::is_invalid_2(123_123), true);
        assert_eq!(SolutionDay2::is_invalid_2(1231_234), false);
        assert_eq!(SolutionDay2::is_invalid_2(1231_1231), true);
        assert_eq!(SolutionDay2::is_invalid_2(12312_3456), false);
        assert_eq!(SolutionDay2::is_invalid_2(7777_7777), true);
        assert_eq!(SolutionDay2::is_invalid_2(0), false);
    }

    #[test]
    fn test_solve_1() {
        let result = SolutionDay2::solve_1("src/day_2/input_test.txt");
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay2::solve_2("src/day_2/input_test.txt");
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay2::solve_1("src/day_2/input.txt");
        assert_eq!(result, 20223751480);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay2::solve_2("src/day_2/input.txt");
        assert_eq!(result, 30260171216);
    }
}
