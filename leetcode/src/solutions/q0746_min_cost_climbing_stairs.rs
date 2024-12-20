/* LeetCode Problem Question 0746: Min Cost Climbing Stairs

You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.

You can either start from the step with index 0, or the step with index 1.

Return the minimum cost to reach the top of the floor.

https://leetcode.com/problems/min-cost-climbing-stairs/
*/

pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() <= 2 {
            return *cost.iter().min().unwrap_or(&0);
        }

        let mut cache = vec![0; cost.len()];
        cache[0] = cost[0];
        cache[1] = cost[1];

        for (mut index, step_cost) in cost.iter().skip(2).enumerate() {
            index += 2;
            cache[index] = step_cost + std::cmp::min(cache[index - 1], cache[index - 2]);
        }

        *cache.iter().rev().take(2).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![4, 1]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![]), 0);
    }
}
