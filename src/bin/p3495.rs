//! # 3495. Minimum Operations to Make Array Elements Zero
//!
//! **Difficulty**: Hard
//!
//! You are given a 2D array queries, where queries[i] is of the form [l, r].
//! Each queries[i] defines an array of integers nums consisting of elements
//! ranging from l to r, both inclusive.
//!
//! In one operation, you can:
//! - Select two integers a and b from the array.
//! - Replace them with floor(a / 4) and floor(b / 4).
//!
//! Your task is to determine the minimum number of operations required to
//! reduce all elements of the array to zero for each query. Return the sum
//! of the results for all queries.
//!
//! **Example 1:**
//! ```
//! Input: queries = [[1,2],[2,4]]
//! Output: 3
//! ```
//!
//! **Example 2:**
//! ```
//! Input: queries = [[2,6]]
//! Output: 4
//! ```
//!
//! ## Approach
//!
//! The key insight is that to reduce a number n to 0 by repeated division by 4,
//! we need floor(log₄(n)) + 1 operations. We can group numbers based on how many
//! divisions they need:
//!
//! - Numbers in [1, 3] (4⁰ to 4¹-1) require 1 division each
//! - Numbers in [4, 15] (4¹ to 4²-1) require 2 divisions each
//! - Numbers in [16, 63] (4² to 4³-1) require 3 divisions each
//! - And so on...
//!
//! Since 4¹⁶ > 10⁹, we need at most 16 intervals to cover all possible values.
//!
//! For each query [l, r]:
//! 1. For each interval [4^(d-1), 4^d - 1], find overlap with [l, r]
//! 2. Add (overlap_length × d) to total operations needed
//! 3. Since we process pairs, final answer is ceil(total_operations / 2)

struct Solution;

impl Solution {
    /// Main solution using mathematical grouping approach
    ///
    /// Time Complexity: O(queries.len())
    /// Space Complexity: O(1)
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut total = 0i64;

        for query in queries {
            let l = query[0] as i64;
            let r = query[1] as i64;
            total += Self::solve_query(l, r);
        }

        total
    }

    /// Solve a single query [l, r]
    ///
    /// Groups numbers by division requirements and calculates total operations needed.
    ///
    /// The algorithm works as follows:
    /// 1. Iterate through intervals [4^(d-1), 4^d - 1] for d = 1, 2, 3, ...
    /// 2. For each interval, find how many numbers from [l, r] fall into it
    /// 3. Each number in interval d requires d divisions to reach 0
    /// 4. Sum up all required divisions across all intervals
    /// 5. Since we process pairs, return ceil(total_divisions / 2)
    fn solve_query(l: i64, r: i64) -> i64 {
        let mut total_operations = 0i64;
        let mut prev = 1i64; // Start of current interval: 4^(d-1)
        let mut d = 1i64;    // Number of divisions required for current interval

        // Process intervals until we cover all numbers up to r
        // Since 4^16 > 10^9, we need at most 16 intervals
        while prev <= r {
            let cur = prev * 4; // Start of next interval: 4^d
            let interval_end = cur - 1; // End of current interval: 4^d - 1

            // Find overlap between query range [l, r] and current interval [prev, interval_end]
            let overlap_start = l.max(prev);
            let overlap_end = r.min(interval_end);

            if overlap_start <= overlap_end {
                // Number of elements in the overlap
                let count = overlap_end - overlap_start + 1;
                // Each element in this interval needs exactly d divisions
                total_operations += count * d;
            }

            // Move to next interval: [4^d, 4^(d+1) - 1]
            prev = cur;
            d += 1;
        }

        // Since we can process 2 numbers at a time, we need ceil(total_operations / 2)
        // Using integer arithmetic: ceil(a/b) = (a + b - 1) / b = (a + 1) / 2 for b=2
        (total_operations + 1) / 2
    }

    /// Helper function to calculate how many divisions a number needs
    #[allow(dead_code)]
    fn operations_needed(mut num: i64) -> i64 {
        let mut ops = 0i64;
        while num > 0 {
            num /= 4;
            ops += 1;
        }
        ops
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![1, 2], vec![2, 4]], 3),
        (vec![vec![2, 6]], 4),
        (vec![vec![1, 4]], 3),
    ];

    for (i, (queries, expected)) in test_cases.iter().enumerate() {
        println!("Example {}: queries = {:?}", i + 1, queries);
        let result = Solution::min_operations(queries.clone());
        println!("Output: {}", result);
        println!("Expected: {}", expected);
        println!("Passed: {}", result == *expected as i64);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let queries = vec![vec![1, 2], vec![2, 4]];
        assert_eq!(Solution::min_operations(queries), 3);
    }

    #[test]
    fn test_example2() {
        let queries = vec![vec![2, 6]];
        assert_eq!(Solution::min_operations(queries), 4);
    }

    #[test]
    fn test_single_number() {
        let queries = vec![vec![4, 4]];
        let result = Solution::min_operations(queries);
        assert_eq!(result, 1); // 4 needs 2 operations, ceil(2/2) = 1
    }

    #[test]
    fn test_operations_needed() {
        assert_eq!(Solution::operations_needed(1), 1);
        assert_eq!(Solution::operations_needed(2), 1);
        assert_eq!(Solution::operations_needed(3), 1);
        assert_eq!(Solution::operations_needed(4), 2);
        assert_eq!(Solution::operations_needed(15), 2);
        assert_eq!(Solution::operations_needed(16), 3);
    }

    #[test]
    fn test_interval_grouping() {
        // Test that our grouping logic works correctly
        // [1,2] should give us:
        // - Numbers 1,2 are in interval [1,3] and need 1 operation each
        // - Total operations = 2, ceil(2/2) = 1
        let queries = vec![vec![1, 2]];
        assert_eq!(Solution::min_operations(queries), 1);

        // [2,4] should give us:
        // - Number 2,3 are in interval [1,3] and need 1 operation each = 2
        // - Number 4 is in interval [4,15] and needs 2 operations = 2
        // - Total operations = 4, ceil(4/2) = 2
        let queries = vec![vec![2, 4]];
        assert_eq!(Solution::min_operations(queries), 2);
    }

    #[test]
    fn test_large_range() {
        // Test with a larger range to verify intervals work correctly
        let queries = vec![vec![1, 16]];
        // [1,3]: 3 numbers * 1 = 3
        // [4,15]: 12 numbers * 2 = 24
        // [16,16]: 1 number * 3 = 3
        // Total = 30, ceil(30/2) = 15
        assert_eq!(Solution::min_operations(queries), 15);
    }

    #[test]
    fn test_example2_breakdown() {
        // Detailed verification of Example 2: [2,6]
        // This shows exactly how the mathematical grouping works

        // Range [2,6] contains numbers: 2, 3, 4, 5, 6

        // Interval 1: [1,3] contains numbers needing 1 division each
        // From [2,6]: numbers 2,3 → 2 numbers × 1 division = 2 operations

        // Interval 2: [4,15] contains numbers needing 2 divisions each
        // From [2,6]: numbers 4,5,6 → 3 numbers × 2 divisions = 6 operations

        // Total operations needed: 2 + 6 = 8
        // Since we process pairs: ceil(8/2) = 4

        let queries = vec![vec![2, 6]];
        assert_eq!(Solution::min_operations(queries), 4);

        // Verify the manual calculation matches our solve_query function
        assert_eq!(Solution::solve_query(2, 6), 4);
    }
}
