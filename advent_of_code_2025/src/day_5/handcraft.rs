pub struct SolutionDay5;
use std::{
    cmp::{max, min},
    fs,
};

impl SolutionDay5 {
    fn read_input(filename: &str) -> (Vec<String>, Vec<u64>) {
        let input = fs::read_to_string(filename).unwrap();
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let separator_index = lines.iter().position(|line| line == "").unwrap();

        (
            lines[..separator_index].to_vec(),
            lines[separator_index + 1..]
                .to_vec()
                .iter()
                .map(|line| line.parse::<u64>().unwrap())
                .collect(),
        )
    }

    fn merge_ranges(
        ranges: &mut Vec<[u64; 2]>,
        update_index: usize,
        update_start: u64,
        update_end: u64,
    ) {
        let mut popping_ranges = Vec::new();

        let mut next_start = min(update_start, ranges[update_index][0]);
        let mut next_end = max(update_end, ranges[update_index][1]);

        // going forward
        if update_index < ranges.len() - 1 {
            for i in update_index + 1..ranges.len() {
                let next_range = ranges[i];
                if next_range[0] <= update_end {
                    popping_ranges.push(i);
                    next_end = next_range[1];
                } else {
                    break;
                }
            }
        }

        // going backward
        if update_index > 0 {
            for i in (0..(update_index - 1)).rev() {
                let next_range = ranges[i];
                if next_range[1] >= update_start {
                    popping_ranges.push(i);
                    next_start = next_range[0];
                } else {
                    break;
                }
            }
        }

        // update the new range
        ranges[update_index] = [next_start, next_end];

        // pop the ranges
        for popping_range in popping_ranges {
            ranges.remove(popping_range as usize);
        }
    }

    pub fn solve_1(filename: &str) -> u64 {
        let (fresh_ingredient_ranges, available_ingredient_ids) = Self::read_input(filename);

        // vector of [start, end]
        // this is the merged ranges
        let mut fresh_ingredient_ids: Vec<[u64; 2]> = Vec::new();

        for range_line in fresh_ingredient_ranges {
            let parts = range_line.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();

            // initial push
            if fresh_ingredient_ids.is_empty() {
                fresh_ingredient_ids.push([start, end]);
                continue;
            }

            // for each range, we check if
            // the start is within the range of the element before it
            // or the end is within the range of the element after it
            let mut inserted = false;
            for i in 0..fresh_ingredient_ids.len() {
                let current_range = fresh_ingredient_ids[i];
                // 4 cases when we can merge the new range with the current range
                // 1. the new range is entirely within the current range
                // 2. the the current range is entirely within the new range
                // 3. the new range is overlapping with the current range from the start (2nd half of the existing range)
                // 4. the new range is overlapping with the current range from the end (1st half of the existing range)
                if (start <= current_range[0] && end >= current_range[1])
                    || (start >= current_range[0] && end <= current_range[1])
                    || (start >= current_range[0] && start <= current_range[1])
                    || (end >= current_range[0] && end <= current_range[1])
                {
                    // update the overlapping range
                    Self::merge_ranges(&mut fresh_ingredient_ids, i, start, end);
                    inserted = true;
                    break;
                }

                // we can safely check the end with the start of the current range since if
                // the start isn't larger than the
                if end < current_range[0] {
                    // insert the new range at the current index
                    fresh_ingredient_ids.insert(i, [start, end]);
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                fresh_ingredient_ids.push([start, end]);
            }
        }

        let mut fresh_ingredient_count = 0;
        for available_ingredient_id in available_ingredient_ids {
            for fresh_ingredient_range in fresh_ingredient_ids.iter() {
                if available_ingredient_id >= fresh_ingredient_range[0]
                    && available_ingredient_id <= fresh_ingredient_range[1]
                {
                    fresh_ingredient_count += 1;
                    break;
                }
            }
        }
        fresh_ingredient_count
    }

    pub fn solve_2(filename: &str) -> u64 {
        let (fresh_ingredient_ranges, _) = Self::read_input(filename);
        let mut fresh_ingredient_ids: Vec<[u64; 2]> = Vec::new();

        for range_line in fresh_ingredient_ranges {
            let parts = range_line.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();

            // initial push
            if fresh_ingredient_ids.is_empty() {
                fresh_ingredient_ids.push([start, end]);
                continue;
            }

            let mut inserted = false;
            for i in 0..fresh_ingredient_ids.len() {
                let current_range = fresh_ingredient_ids[i];
                if (start <= current_range[0] && end >= current_range[1])
                    || (start >= current_range[0] && end <= current_range[1])
                    || (start >= current_range[0] && start <= current_range[1])
                    || (end >= current_range[0] && end <= current_range[1])
                {
                    // update the overlapping range
                    Self::merge_ranges(&mut fresh_ingredient_ids, i, start, end);
                    inserted = true;
                    break;
                }
                if end < current_range[0] {
                    // insert the new range at the current index
                    fresh_ingredient_ids.insert(i, [start, end]);
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                fresh_ingredient_ids.push([start, end]);
            }
        }

        let mut total_fresh_ingredient_count = 0;
        for fresh_ingredient_range in fresh_ingredient_ids.iter() {
            total_fresh_ingredient_count +=
                fresh_ingredient_range[1] - fresh_ingredient_range[0] + 1;
        }

        total_fresh_ingredient_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay5::solve_1("src/day_5/input_test.txt");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay5::solve_2("src/day_5/input_test.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay5::solve_1("src/day_5/input.txt");
        assert_eq!(result, 520);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay5::solve_2("src/day_5/input.txt");
        assert_eq!(result, 347338785050515);
    }
}
