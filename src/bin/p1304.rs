//! Problem 1304: Find N Unique Integers Sum up to Zero
//!
//! Given an integer n, return any array containing n unique integers such that they add up to 0.
//!
//! Example 1:
//! Input: n = 5
//! Output: [-7,-1,1,3,4]
//! Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
//!
//! Example 2:
//! Input: n = 3
//! Output: [-1,0,1]
//!
//! Example 3:
//! Input: n = 1
//! Output: [0]

struct Solution;

impl Solution {
    /// Find n unique integers that sum up to zero
    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(n) for the result array
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);

        // For even n: create pairs (1, -1), (2, -2), ..., (n/2, -n/2)
        // For odd n: create pairs plus add 0 at the end
        for i in 1..=n/2 {
            result.push(i);
            result.push(-i);
        }

        // If n is odd, add 0 to make the count exact
        if n % 2 == 1 {
            result.push(0);
        }

        result
    }

    /// Alternative solution using a different pattern
    #[allow(dead_code)]
    pub fn sum_zero_alt(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);

        // Add numbers 1 to n-1
        for i in 1..n {
            result.push(i);
        }

        // Add the negative sum of all previous numbers
        let sum: i32 = result.iter().sum();
        result.push(-sum);

        result
    }
}

fn main() {
    let test_cases = vec![
        (5, vec![-2, -1, 0, 1, 2]), // Expected pattern (any valid answer)
        (3, vec![-1, 0, 1]),
        (1, vec![0]),
        (4, vec![-2, -1, 1, 2]),
        (2, vec![-1, 1]),
    ];

    for (i, (input, _expected)) in test_cases.iter().enumerate() {
        println!("Example {}: n = {}", i + 1, input);
        let result = Solution::sum_zero(*input);
        let sum: i32 = result.iter().sum();
        println!("Output: {:?}", result);
        println!("Sum: {}", sum);
        println!("Length: {}", result.len());
        println!("Passed: {} (sum=0, length=n, unique elements)",
                 sum == 0 && result.len() == *input as usize &&
                 result.iter().collect::<std::collections::HashSet<_>>().len() == result.len());
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn validate_result(result: &[i32], n: i32) -> bool {
        // Check length
        if result.len() != n as usize {
            return false;
        }

        // Check sum is zero
        if result.iter().sum::<i32>() != 0 {
            return false;
        }

        // Check all elements are unique
        let unique_count = result.iter().collect::<HashSet<_>>().len();
        unique_count == result.len()
    }

    #[test]
    fn test_example1() {
        let result = Solution::sum_zero(5);
        assert!(validate_result(&result, 5));
    }

    #[test]
    fn test_example2() {
        let result = Solution::sum_zero(3);
        assert!(validate_result(&result, 3));
    }

    #[test]
    fn test_example3() {
        let result = Solution::sum_zero(1);
        assert!(validate_result(&result, 1));
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_edge_cases() {
        // Test small even number
        let result = Solution::sum_zero(2);
        assert!(validate_result(&result, 2));

        // Test small odd number
        let result = Solution::sum_zero(3);
        assert!(validate_result(&result, 3));

        // Test larger numbers
        for n in 4..=20 {
            let result = Solution::sum_zero(n);
            assert!(validate_result(&result, n), "Failed for n = {}", n);
        }
    }

    #[test]
    fn test_alternative_solution() {
        for n in 1..=10 {
            let result1 = Solution::sum_zero(n);
            let result2 = Solution::sum_zero_alt(n);

            assert!(validate_result(&result1, n));
            assert!(validate_result(&result2, n));
        }
    }
}
