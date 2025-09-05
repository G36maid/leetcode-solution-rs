//! # 2749. Minimum Operations to Make the Integer Zero
//!
//! You are given two integers num1 and num2.
//!
//! In one operation, you can choose integer i in the range [0, 60] and subtract 2^i + num2 from num1.
//!
//! Return the integer denoting the minimum number of operations needed to make num1 equal to 0.
//! If it is impossible to make num1 equal to 0, return -1.
//!
//! ## Example 1:
//! Input: num1 = 3, num2 = -2
//! Output: 3
//! Explanation: We can make 3 equal to 0 with the following operations:
//! - We choose i = 2 and subtract 2^2 + (-2) from 3, 3 - (4 + (-2)) = 1.
//! - We choose i = 2 and subtract 2^2 + (-2) from 1, 1 - (4 + (-2)) = -1.
//! - We choose i = 0 and subtract 2^0 + (-2) from -1, (-1) - (1 + (-2)) = 0.
//!
//! ## Example 2:
//! Input: num1 = 5, num2 = 7
//! Output: -1
//! Explanation: It can be proven, that it is impossible to make 5 equal to 0 with the given operation.

struct Solution;

impl Solution {
    /// Find minimum operations to make num1 equal to 0
    ///
    /// The key insight: after k operations, we need sum of k powers of 2 = num1 - k*num2
    /// This is possible iff: popcount(target) ≤ k ≤ target and target > 0
    ///
    /// Time Complexity: O(log(max(num1, |num2|)))
    /// Space Complexity: O(1)
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let (num1, num2) = (num1 as i64, num2 as i64);

        (1..=Self::max_operations(num2))
            .find(|&operations| Self::can_achieve_zero_in_k_operations(num1, num2, operations))
            .map(|k| k as i32)
            .unwrap_or(-1)
    }

    /// Check if we can make num1 zero using exactly k operations
    fn can_achieve_zero_in_k_operations(num1: i64, num2: i64, k: i64) -> bool {
        let target = num1 - k * num2;

        target > 0 && Self::can_represent_as_sum_of_k_powers(target, k)
    }

    /// Check if a positive number can be represented as sum of exactly k powers of 2
    fn can_represent_as_sum_of_k_powers(target: i64, k: i64) -> bool {
        let min_powers_needed = target.count_ones() as i64;
        let max_powers_usable = target; // using all 2^0 = 1

        min_powers_needed <= k && k <= max_powers_usable
    }

    /// Determine reasonable upper bound for number of operations
    fn max_operations(num2: i64) -> i64 {
        if num2 < 0 { 64 } else { 50 }
    }
}

fn main() {
    let test_cases = [
        (3, -2, 3),
        (5, 7, -1),
        (1, 1, -1),
        (10, -5, 2),
        (4, 1, 2),
        (112577768, -501662198, 16),
    ];

    test_cases
        .iter()
        .enumerate()
        .for_each(|(i, &(num1, num2, expected))| {
            let result = Solution::make_the_integer_zero(num1, num2);
            println!(
                "Test {}: make_zero({}, {}) = {} (expected: {}) ✓",
                i + 1,
                num1,
                num2,
                result,
                expected
            );
            assert_eq!(result, expected, "Failed test case {}", i + 1);
        });

    println!("\nAll test cases passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_examples() {
        assert_eq!(Solution::make_the_integer_zero(3, -2), 3);
        assert_eq!(Solution::make_the_integer_zero(5, 7), -1);
    }

    #[test]
    fn test_simple_cases() {
        assert_eq!(Solution::make_the_integer_zero(1, 0), 1); // target=1, use 2^0
        assert_eq!(Solution::make_the_integer_zero(8, 0), 1); // target=8, use 2^3
        assert_eq!(Solution::make_the_integer_zero(1, 1), -1); // target=0, impossible
    }

    #[test]
    fn test_negative_num2() {
        assert_eq!(Solution::make_the_integer_zero(15, -1), 1); // target=16, use 2^4
        assert_eq!(Solution::make_the_integer_zero(2, 1), 1); // target=1, use 2^0
        assert_eq!(Solution::make_the_integer_zero(7, 1), 2); // target=5, popcount=2
    }

    #[test]
    fn test_impossible_cases() {
        assert_eq!(Solution::make_the_integer_zero(1, 2), -1); // all targets negative
        assert_eq!(Solution::make_the_integer_zero(5, 7), -1); // all targets negative
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::make_the_integer_zero(4, 1), 2); // target=2, use 2^1
        assert_eq!(Solution::make_the_integer_zero(10, -5), 2); // target=20, popcount=2
    }

    #[test]
    fn test_large_numbers() {
        // The user-reported case that was initially failing
        assert_eq!(Solution::make_the_integer_zero(112577768, -501662198), 16);

        // Additional large number test
        let result = Solution::make_the_integer_zero(1000000, -1000);
        assert!(result > 0, "Should find a valid solution");
    }
}
