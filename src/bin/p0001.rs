//! Problem 1: Two Sum
//!
//! Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.
//!
//! Example 1:
//! Input: nums = [2,7,11,15], target = 9
//! Output: [0,1]
//! Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//!
//! Example 2:
//! Input: nums = [3,2,4], target = 6
//! Output: [1,2]
//!
//! Example 3:
//! Input: nums = [3,3], target = 6
//! Output: [0,1]

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Two Sum solution using HashMap for O(n) time complexity
    ///
    /// Time Complexity: O(n) where n is the length of nums
    /// Space Complexity: O(n) for the HashMap
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&complement_index) = map.get(&complement) {
                return vec![complement_index as i32, i as i32];
            }

            map.insert(num, i);
        }

        vec![] // Should never reach here given problem constraints
    }

    /// Brute force solution for comparison (O(n²) time complexity)
    #[allow(dead_code)]
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

fn main() {
    // Test cases
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
    ];

    for (i, (nums, target, expected)) in test_cases.iter().enumerate() {
        println!("Example {}: nums = {:?}, target = {}", i + 1, nums, target);
        let result = Solution::two_sum(nums.clone(), *target);
        println!("Output: {:?}", result);
        println!("Expected: {:?}", expected);
        println!("Passed: {}", result == *expected);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_example1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_example2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_example3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_brute_force() {
        assert_eq!(
            Solution::two_sum_brute_force(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
        assert_eq!(Solution::two_sum_brute_force(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_brute_force(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_edge_cases() {
        // Test with minimum size array
        assert_eq!(Solution::two_sum(vec![1, 2], 3), vec![0, 1]);

        // Test with negative numbers
        assert_eq!(Solution::two_sum(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]);
    }

    #[test]
    fn test_both_solutions_consistency() {
        let test_cases = vec![(vec![2, 7, 11, 15], 9), (vec![3, 2, 4], 6), (vec![3, 3], 6)];

        for (nums, target) in test_cases {
            let result1 = Solution::two_sum(nums.clone(), target);
            let result2 = Solution::two_sum_brute_force(nums.clone(), target);

            // Both should find valid indices
            assert_eq!(
                nums[result1[0] as usize] + nums[result1[1] as usize],
                target
            );
            assert_eq!(
                nums[result2[0] as usize] + nums[result2[1] as usize],
                target
            );
        }
    }
}
