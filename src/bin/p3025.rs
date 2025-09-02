//! Problem 3025: Find the Number of Ways to Place People I
//!
//! You are given a 2D array points of size n x 2 representing integer coordinates of some points on a 2D plane,
//! where points[i] = [xi, yi].
//!
//! Count the number of pairs of points (A, B), where:
//! - A is on the upper left side of B, and
//! - there are no other points in the rectangle (or line) they make (including the border).
//!
//! Return the count.
//!
//! Example 1:
//! Input: points = [[1,1],[2,2],[3,3]]
//! Output: 0
//! Explanation: There is no way to choose A and B so A is on the upper left side of B.
//!
//! Example 2:
//! Input: points = [[6,2],[4,4],[2,6]]
//! Output: 2
//! Explanation: Two valid pairs exist.
//!
//! Example 3:
//! Input: points = [[3,1],[1,3],[1,1]]
//! Output: 2
//! Explanation: Two valid pairs exist.

struct Solution;

impl Solution {
    /// Find the number of ways to place people
    ///
    /// Algorithm:
    /// For each pair of points (A, B), check if:
    /// 1. A is on the upper left side of B (A.x <= B.x and A.y >= B.y, with at least one strict inequality)
    /// 2. No other points exist in the rectangle/line formed by A and B (including borders)
    ///
    /// Time Complexity: O(n³) where n is the number of points
    /// Space Complexity: O(1) excluding input
    ///
    /// The solution checks all n² pairs and for each pair checks all n points for containment.
    pub fn number_of_people_places(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut count = 0;

        // Check all pairs of points (A, B)
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let a = &points[i];
                let b = &points[j];

                // Check if A is on the upper left side of B
                if Self::is_upper_left(a, b) {
                    // Check if the rectangle/line is empty
                    if Self::is_rectangle_empty(a, b, &points, i, j) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Check if point A is on the upper left side of point B
    /// A is upper left of B if: A.x <= B.x and A.y >= B.y and (A.x < B.x or A.y > B.y)
    fn is_upper_left(a: &[i32], b: &[i32]) -> bool {
        let (ax, ay) = (a[0], a[1]);
        let (bx, by) = (b[0], b[1]);

        ax <= bx && ay >= by && (ax < bx || ay > by)
    }

    /// Check if the rectangle/line formed by points A and B contains no other points
    fn is_rectangle_empty(a: &[i32], b: &[i32], points: &[Vec<i32>], i: usize, j: usize) -> bool {
        let (ax, ay) = (a[0], a[1]);
        let (bx, by) = (b[0], b[1]);

        // Check all other points
        for (k, point) in points.iter().enumerate() {
            if k == i || k == j {
                continue;
            }

            let (cx, cy) = (point[0], point[1]);

            // Check if point C is inside or on the boundary of the rectangle
            if Self::is_point_in_rectangle(cx, cy, ax, ay, bx, by) {
                return false;
            }
        }

        true
    }

    /// Check if point (cx, cy) is inside or on the boundary of rectangle defined by
    /// upper-left corner (ax, ay) and lower-right corner (bx, by)
    fn is_point_in_rectangle(cx: i32, cy: i32, ax: i32, ay: i32, bx: i32, by: i32) -> bool {
        // The rectangle has corners at (ax, ay) and (bx, by)
        // Since A is upper-left of B: ax <= bx and ay >= by
        // So the rectangle spans from (ax, by) to (bx, ay)

        // Check if point is within the rectangle bounds (inclusive of borders)
        ax <= cx && cx <= bx && by <= cy && cy <= ay
    }
}

fn main() {
    // Test cases
    let test_cases = vec![
        vec![vec![1, 1], vec![2, 2], vec![3, 3]],
        vec![vec![6, 2], vec![4, 4], vec![2, 6]],
        vec![vec![3, 1], vec![1, 3], vec![1, 1]],
    ];

    for (i, points) in test_cases.iter().enumerate() {
        println!("Example {}: points = {:?}", i + 1, points);
        let result = Solution::number_of_people_places(points.clone());
        println!("Output: {}", result);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::number_of_people_places(points), 0);
    }

    #[test]
    fn test_example2() {
        let points = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
        assert_eq!(Solution::number_of_people_places(points), 2);
    }

    #[test]
    fn test_example3() {
        let points = vec![vec![3, 1], vec![1, 3], vec![1, 1]];
        assert_eq!(Solution::number_of_people_places(points), 2);
    }

    #[test]
    fn test_is_upper_left() {
        // Test upper left relationship
        assert!(Solution::is_upper_left(&[1, 3], &[3, 1])); // A(1,3) is upper-left of B(3,1)
        assert!(Solution::is_upper_left(&[2, 2], &[3, 1])); // A(2,2) is upper-left of B(3,1)
        assert!(!Solution::is_upper_left(&[3, 1], &[1, 3])); // A(3,1) is not upper-left of B(1,3)
        assert!(!Solution::is_upper_left(&[2, 2], &[2, 2])); // Same point

        // Test edge cases - same x or same y
        assert!(Solution::is_upper_left(&[1, 3], &[1, 1])); // Same x, A higher
        assert!(Solution::is_upper_left(&[1, 2], &[3, 2])); // Same y, A to the left
    }

    #[test]
    fn test_is_point_in_rectangle() {
        // Rectangle from (1, 3) to (3, 1)
        assert!(Solution::is_point_in_rectangle(2, 2, 1, 3, 3, 1)); // Inside
        assert!(Solution::is_point_in_rectangle(1, 1, 1, 3, 3, 1)); // On corner
        assert!(Solution::is_point_in_rectangle(2, 3, 1, 3, 3, 1)); // On top edge
        assert!(Solution::is_point_in_rectangle(3, 2, 1, 3, 3, 1)); // On right edge
        assert!(!Solution::is_point_in_rectangle(0, 2, 1, 3, 3, 1)); // Outside left
        assert!(!Solution::is_point_in_rectangle(4, 2, 1, 3, 3, 1)); // Outside right
        assert!(!Solution::is_point_in_rectangle(2, 0, 1, 3, 3, 1)); // Outside below
        assert!(!Solution::is_point_in_rectangle(2, 4, 1, 3, 3, 1)); // Outside above
    }

    #[test]
    fn test_two_points() {
        let points = vec![vec![1, 3], vec![3, 1]];
        assert_eq!(Solution::number_of_people_places(points), 1);
    }

    #[test]
    fn test_all_same_line() {
        let points = vec![vec![1, 1], vec![2, 1], vec![3, 1]];
        // (1,1) -> (2,1): valid, no point between them
        // (1,1) -> (3,1): invalid because (2,1) is between them on the line
        // (2,1) -> (3,1): valid, no point between them
        // All points have same y, so pairs are valid if A.x < B.x and no points between
        assert_eq!(Solution::number_of_people_places(points), 2);
    }

    #[test]
    fn test_edge_case_vertical_line() {
        let points = vec![vec![1, 1], vec![1, 2], vec![1, 3]];
        // (1,3) -> (1,2): valid, no point between them
        // (1,3) -> (1,1): invalid because (1,2) is between them
        // (1,2) -> (1,1): valid, no point between them
        assert_eq!(Solution::number_of_people_places(points), 2);
    }

    #[test]
    fn test_rectangle_with_internal_point() {
        let points = vec![vec![0, 2], vec![2, 0], vec![1, 1]];
        // (0,2) -> (2,0): invalid because (1,1) is inside the rectangle
        // (0,2) -> (1,1): valid because (2,0) is not inside rectangle from (0,2) to (1,1)
        // (1,1) -> (2,0): valid because (0,2) is not inside rectangle from (1,1) to (2,0)
        assert_eq!(Solution::number_of_people_places(points), 2);
    }
}
