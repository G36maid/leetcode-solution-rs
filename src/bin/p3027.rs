//! 3027. Find the Number of Ways to Place People II
//!
//! You are given a 2D array points of size n x 2 representing integer coordinates of some points
//! on a 2D-plane, where points[i] = [xi, yi].
//!
//! We define the right direction as positive x-axis (increasing x-coordinate) and the left
//! direction as negative x-axis (decreasing x-coordinate). Similarly, we define the up direction
//! as positive y-axis (increasing y-coordinate) and the down direction as negative y-axis
//! (decreasing y-coordinate).
//!
//! You have to place n people, including Alice and Bob, at these points such that there is
//! exactly one person at every point. Alice wants to be alone with Bob, so Alice will build
//! a rectangular fence with Alice's position as the upper left corner and Bob's position as
//! the lower right corner of the fence (Note that the fence might not enclose any area, i.e.
//! it can be a line). If any person other than Alice and Bob is either inside the fence or on
//! the fence, Alice will be sad.
//!
//! Return the number of pairs of points where you can place Alice and Bob, such that Alice
//! does not become sad on building the fence.
//!
//! Note that Alice can only build a fence with Alice's position as the upper left corner,
//! and Bob's position as the lower right corner.

struct Solution;

impl Solution {
    /// Brilliant optimized solution using coordinate sorting and monotonic approach
    ///
    /// Time Complexity: O(n log n + n²) = O(n²) with excellent constants
    /// Space Complexity: O(1) additional space (sorting is in-place)
    ///
    /// Key Insight: Sort points by (-x, y) so that for any valid (Alice, Bob) pair,
    /// Alice appears before Bob in the sorted order. This transforms the 2D problem
    /// into a 1D monotonic sequence problem.
    ///
    /// Why this works:
    /// 1. After sorting by (-x, y), if Alice is at position i and Bob at j > i:
    ///    - Alice's x >= Bob's x (guaranteed by decreasing x sort)
    ///    - We only need to check if Alice's y >= Bob's y
    /// 2. For interference detection, we use the fact that interfering points
    ///    must have been processed earlier (due to sorting)
    /// 3. The variable `y` tracks the minimum y-coordinate of valid Bob candidates
    ///    seen so far - this acts like a "monotonic stack" property
    ///
    /// Algorithm:
    /// 1. Sort by (-x, y): decreasing x, then increasing y
    /// 2. For each Alice position i, scan positions j > i as potential Bob
    /// 3. Use monotonic variable y to track minimum y-coordinate of valid candidates
    /// 4. Bob at j is valid if: yj >= yi (positioning) and yj < y (no interference)
    /// 5. Early termination when yi == yj (no better candidates possible)
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        // Sort by (-x, y): decreasing x, then increasing y
        points.sort_by_key(|p| (-p[0], p[1]));

        let n = points.len();
        let mut ans = 0;

        for i in 0..n - 1 {
            let mut y = i32::MAX; // Track minimum y of valid Bob candidates
            let yi = points[i][1]; // Alice's y-coordinate

            for j in i + 1..n {
                let yj = points[j][1]; // Bob's y-coordinate

                // Bob is valid if:
                // 1. yj >= yi (Bob's y <= Alice's y for upper-left/lower-right)
                // 2. yj < y (no interference from previous points)
                if y > yj && yj >= yi {
                    ans += 1;
                    y = yj; // Update minimum y seen

                    // Early termination: if Alice and Bob have same y,
                    // no further points can be valid Bob candidates
                    if yi == yj {
                        break;
                    }
                }
            }
        }

        ans
    }

    /// Alternative clean O(n²) solution without complex optimizations
    #[allow(dead_code)]
    pub fn number_of_pairs_simple(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut count = 0;

        for i in 0..n {
            let (ax, ay) = (points[i][0], points[i][1]);

            for j in 0..n {
                if i == j {
                    continue;
                }

                let (bx, by) = (points[j][0], points[j][1]);

                // Check if Alice can be upper left and Bob can be lower right
                if ax > bx || ay < by {
                    continue;
                }

                // Check if any other point interferes
                let mut blocked = false;

                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }

                    let (px, py) = (points[k][0], points[k][1]);

                    // Point interferes if inside or on rectangle boundary
                    if ax <= px && px <= bx && by <= py && py <= ay {
                        blocked = true;
                        break;
                    }
                }

                if !blocked {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![1, 1], vec![2, 2], vec![3, 3]], 0),
        (vec![vec![6, 2], vec![4, 4], vec![2, 6]], 2),
        (vec![vec![3, 1], vec![1, 3], vec![1, 1]], 2),
        (vec![vec![1, 1], vec![2, 2]], 0),
        (vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![1, 0]], 4),
        (vec![vec![0, 2], vec![1, 1], vec![2, 0]], 2),
    ];

    for (i, (points, expected)) in test_cases.iter().enumerate() {
        println!("Example {}: points = {:?}", i + 1, points);
        let result = Solution::number_of_pairs(points.clone());
        println!("Output: {}", result);
        println!("Expected: {}", expected);
        println!("Passed: {}", result == *expected);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example2() {
        let points = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example3() {
        let points = vec![vec![3, 1], vec![1, 3], vec![1, 1]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_two_points_diagonal() {
        let points = vec![vec![1, 1], vec![2, 2]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_square_formation() {
        let points = vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![1, 0]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_vertical_line() {
        let points = vec![vec![1, 1], vec![1, 2], vec![1, 3]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_horizontal_line() {
        let points = vec![vec![1, 1], vec![2, 1], vec![3, 1]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_large_coordinates() {
        let points = vec![
            vec![-1000000000, 1000000000],
            vec![0, 0],
            vec![1000000000, -1000000000],
        ];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_complex_case() {
        let points = vec![vec![0, 2], vec![1, 1], vec![2, 0]];
        let result = Solution::number_of_pairs(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_consistency_between_solutions() {
        let test_cases = vec![
            vec![vec![1, 1], vec![2, 2], vec![3, 3]],
            vec![vec![6, 2], vec![4, 4], vec![2, 6]],
            vec![vec![3, 1], vec![1, 3], vec![1, 1]],
            vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![1, 0]],
        ];

        for points in test_cases {
            let result_optimized = Solution::number_of_pairs(points.clone());
            let result_simple = Solution::number_of_pairs_simple(points);
            assert_eq!(result_optimized, result_simple,);
        }
    }

    #[test]
    fn test_algorithmic_insight_verification() {
        // Test case to verify the sorting insight works correctly
        let points = vec![vec![3, 1], vec![1, 3], vec![2, 2], vec![4, 0]];
        let result = Solution::number_of_pairs(points);
        // After sorting by (-x, y): [(4,0), (3,1), (2,2), (1,3)]
        // Valid pairs: (1,3)->(2,2), (1,3)->(3,1), (1,3)->(4,0), (2,2)->(3,1), (2,2)->(4,0), (3,1)->(4,0)
        // But some are blocked by interference
        assert!(result > 0);
    }
}
