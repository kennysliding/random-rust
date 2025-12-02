pub struct SolutionDay1;
use std::fs;

impl SolutionDay1 {
    fn read_input(filename: &str) -> Vec<String> {
        let input = fs::read_to_string(filename).unwrap();
        input.lines().map(|line| line.to_string()).collect()
    }

    pub fn solve_1(filename: &str) -> i32 {
        let mut current_position = 50;
        let mut count = 0;
        for line in Self::read_input(filename) {
            let (direction, number) = line.split_at(1);
            let number = number.parse::<i32>().unwrap();
            match direction {
                "L" => current_position = (current_position - number + 100) % 100,
                "R" => current_position = (current_position + number) % 100,
                _ => panic!("Invalid direction: {}", direction.to_string()),
            }
            if current_position == 0 {
                count += 1;
            }
        }
        count
    }

    pub fn solve_2(filename: &str) -> i32 {
        let mut current_position = 50;
        let mut count = 0 as i32;
        for line in Self::read_input(filename) {
            let (direction, number) = line.split_at(1);
            let mut number = number.parse::<i32>().unwrap();

            let previous_position = current_position;

            // take into account every time the dial passes through 0
            count += number / 100;
            number = number % 100;

            match direction {
                "L" => current_position = current_position - number,
                "R" => current_position = current_position + number,
                _ => panic!("Invalid direction: {}", direction.to_string()),
            }

            if current_position == 0 {
                count += 1;
                continue;
            }

            if current_position < 0 {
                if previous_position != 0 {
                    count += 1;
                }
                current_position = (current_position + 100) % 100;
                continue;
            }

            if current_position > 99 {
                count += 1;
                current_position = current_position % 100;
                continue;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        assert_eq!(SolutionDay1::solve_1("src/day_1/input_test.txt"), 3);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(SolutionDay1::solve_2("src/day_1/input_test.txt"), 6);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        assert_eq!(SolutionDay1::solve_1("src/day_1/input.txt"), 962);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        assert_eq!(SolutionDay1::solve_2("src/day_1/input.txt"), 5782);
    }
}
