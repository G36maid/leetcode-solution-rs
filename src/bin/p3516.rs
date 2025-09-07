//! 3516. Find Closest Person
//!
//! You are given three integers x, y, and z, representing the positions of three people on a number line:
//!
//!     x is the position of Person 1.
//!     y is the position of Person 2.
//!     z is the position of Person 3, who does not move.
//!
//! Both Person 1 and Person 2 move toward Person 3 at the same speed.
//!
//! Determine which person reaches Person 3 first:
//!
//!     Return 1 if Person 1 arrives first.
//!     Return 2 if Person 2 arrives first.
//!     Return 0 if both arrive at the same time.
//!
//! Return the result accordingly.
//!
//! Example 1:
//! Input: x = 2, y = 7, z = 4
//! Output: 1
//! Explanation:
//!     Person 1 is at position 2 and can reach Person 3 (at position 4) in 2 steps.
//!     Person 2 is at position 7 and can reach Person 3 in 3 steps.
//! Since Person 1 reaches Person 3 first, the output is 1.
//!
//! Example 2:
//! Input: x = 2, y = 5, z = 6
//! Output: 2
//! Explanation:
//!     Person 1 is at position 2 and can reach Person 3 (at position 6) in 4 steps.
//!     Person 2 is at position 5 and can reach Person 3 in 1 step.
//! Since Person 2 reaches Person 3 first, the output is 2.
//!
//! Example 3:
//! Input: x = 1, y = 5, z = 3
//! Output: 0
//! Explanation:
//!     Person 1 is at position 1 and can reach Person 3 (at position 3) in 2 steps.
//!     Person 2 is at position 5 and can reach Person 3 in 2 steps.
//! Since both Person 1 and Person 2 reach Person 3 at the same time, the output is 0.
//!
//! Constraints:
//!     1 <= x, y, z <= 100

struct Solution;

impl Solution {
    /// Determines which person (1 or 2) reaches person 3 first.
    ///
    /// Uses functional programming style with combinators to compare distances.
    /// The time to reach is proportional to the distance.
    ///
    /// Time Complexity: O(1) - The function performs a fixed number of arithmetic operations.
    /// Space Complexity: O(1) - The function uses a constant amount of extra space.
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        [1, 0, 2][((x - z).abs().cmp(&(y - z).abs()) as i8 + 1) as usize]
    }

    /// Alternative functional solution using closure composition and point-free style
    ///
    /// Demonstrates a more concise functional approach using combinators.
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    #[allow(dead_code)]
    pub fn find_closest_alt(x: i32, y: i32, z: i32) -> i32 {
        let distance = |pos: i32| (pos - z).abs();
        let compare_distances = |a: i32, b: i32| distance(a).cmp(&distance(b));

        match compare_distances(x, y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => 2,
            std::cmp::Ordering::Equal => 0,
        }
    }

    /// Pure functional approach using function composition
    ///
    /// Uses higher-order functions and function composition principles.
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    #[allow(dead_code)]
    pub fn find_closest_pure_functional(x: i32, y: i32, z: i32) -> i32 {
        let to_distance = |pos: i32| (pos - z).abs();
        let select_winner = |(dist1, dist2): (i32, i32)| match dist1.cmp(&dist2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => 2,
            std::cmp::Ordering::Equal => 0,
        };

        [x, y]
            .iter()
            .map(|&pos| to_distance(pos))
            .collect::<Vec<_>>()
            .chunks(2)
            .next()
            .map(|chunk| (chunk[0], chunk[1]))
            .map(select_winner)
            .unwrap_or(0)
    }

    /// Most concise functional approach
    ///
    /// Ultra-concise using tuple destructuring and match expression.
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    #[allow(dead_code)]
    pub fn find_closest_concise(x: i32, y: i32, z: i32) -> i32 {
        match ((x - z).abs(), (y - z).abs()) {
            (d1, d2) if d1 < d2 => 1,
            (d1, d2) if d1 > d2 => 2,
            _ => 0,
        }
    }
}

fn main() {
    // Test cases
    let test_cases = vec![
        ((2, 7, 4), 1),
        ((2, 5, 6), 2),
        ((1, 5, 3), 0),
        ((1, 1, 10), 0),
        ((10, 1, 5), 2),
        ((1, 10, 5), 1),
    ];

    test_cases
        .iter()
        .enumerate()
        .map(|(i, ((x, y, z), expected))| {
            let result = Solution::find_closest(*x, *y, *z);
            (i + 1, (x, y, z), result, expected, result == *expected)
        })
        .for_each(|(test_num, (x, y, z), result, expected, passed)| {
            println!("Test case {}: ", test_num);
            println!("Input: x = {}, y = {}, z = {}", x, y, z);
            println!("Output: {:?}", result);
            println!("Expected: {:?}", expected);
            println!("Passed: {}", passed);
            println!();
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::find_closest(2, 7, 4), 1);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::find_closest(2, 5, 6), 2);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::find_closest(1, 5, 3), 0);
    }

    #[test]
    fn test_same_position() {
        assert_eq!(Solution::find_closest(5, 5, 10), 0);
    }

    #[test]
    fn test_z_between_x_y() {
        assert_eq!(Solution::find_closest(1, 10, 5), 1);
    }

    #[test]
    fn test_z_outside_x_y() {
        assert_eq!(Solution::find_closest(2, 4, 10), 2);
    }

    #[test]
    fn test_all_functional_approaches_consistent() {
        let test_cases = [
            (2, 7, 4),
            (2, 5, 6),
            (1, 5, 3),
            (1, 1, 10),
            (10, 1, 5),
            (1, 10, 5),
        ];

        for &(x, y, z) in &test_cases {
            let result1 = Solution::find_closest(x, y, z);
            let result2 = Solution::find_closest_alt(x, y, z);
            let result3 = Solution::find_closest_pure_functional(x, y, z);
            let result4 = Solution::find_closest_concise(x, y, z);

            assert_eq!(
                result1, result2,
                "Alt approach differs for ({}, {}, {})",
                x, y, z
            );
            assert_eq!(
                result1, result3,
                "Pure functional approach differs for ({}, {}, {})",
                x, y, z
            );
            assert_eq!(
                result1, result4,
                "Concise approach differs for ({}, {}, {})",
                x, y, z
            );
        }
    }
}
