pub struct SolutionDay6;
use std::{cmp::max, fs};

impl SolutionDay6 {
    fn read_input(filename: &str) -> Vec<String> {
        let input = fs::read_to_string(filename).unwrap();
        input.lines().map(|line| line.to_string()).collect()
    }

    pub fn solve_1(filename: &str) -> u64 {
        let input = Self::read_input(filename);

        let parsed_input: Vec<Vec<String>> = input
            .iter()
            .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
            .collect();

        let mut total = 0;

        for i in 0..parsed_input[0].len() {
            let operator = parsed_input[parsed_input.len() - 1][i].clone();

            let mut sequence = vec![];

            for j in 0..parsed_input.len() - 1 {
                sequence.push(parsed_input[j][i].parse::<u64>().unwrap());
            }

            match operator.as_str() {
                "+" => total += sequence.iter().sum::<u64>(),
                "*" => total += sequence.iter().product::<u64>(),
                _ => panic!("Invalid operator: {}", operator),
            }
        }
        total
    }

    pub fn solve_2(filename: &str) -> u64 {
        let input = Self::read_input(filename);

        let mut total = 0;

        let mut column_start: usize = 0;
        let mut column_end: usize = 1;

        loop {
            // use the operator row to find the width of a column
            loop {
                if column_end == input[0].len() {
                    break;
                }
                match input[input.len() - 1].chars().nth(column_end).unwrap() {
                    ' ' => column_end += 1,
                    _ => break,
                }
            }

            let operator = input[input.len() - 1].chars().nth(column_start).unwrap();

            // build the sequence of numbers with digits
            let mut sequence = vec![];
            for i in column_start..column_end {
                let mut number_string = String::new();
                for j in 0..input.len() - 1 {
                    let digit = input[j].chars().nth(i).unwrap();
                    number_string.push(digit);
                }
                number_string = number_string.replace(" ", "");
                if number_string.is_empty() {
                    continue;
                }
                let number = number_string.parse::<u64>().unwrap();
                sequence.push(number);
            }

            match operator {
                '+' => total += sequence.iter().sum::<u64>(),
                '*' => total += sequence.iter().product::<u64>(),
                _ => panic!("Invalid operator: {}", operator),
            }
            if column_end == input[0].len() {
                break;
            }
            column_start = column_end;
            column_end += 1;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay6::solve_1("src/day_6/input_test.txt");
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay6::solve_2("src/day_6/input_test.txt");
        assert_eq!(result, 3263827);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay6::solve_1("src/day_6/input.txt");
        assert_eq!(result, 4076006202939);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay6::solve_2("src/day_6/input.txt");
        assert_eq!(result, 7903168391557);
    }
}
