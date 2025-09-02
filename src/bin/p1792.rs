//! 1792. Maximum Average Pass Ratio
//!
//! There is a school that has classes of students and each class will be having a final exam.
//! You are given a 2D integer array classes, where classes[i] = [passi, totali]. You know
//! beforehand that in the ith class, there are totali total students, but only passi number
//! of students will pass the exam.
//!
//! You are also given an integer extraStudents. There are another extraStudents brilliant
//! students that are guaranteed to pass the exam of any class they are assigned to. You want
//! to assign each of the extraStudents students to a class in a way that maximizes the average
//! pass ratio across all the classes.
//!
//! The pass ratio of a class is equal to the number of students of the class that will pass
//! the exam divided by the total number of students of the class. The average pass ratio is
//! the sum of pass ratios of all the classes divided by the number of the classes.
//!
//! Return the maximum possible average pass ratio after assigning the extraStudents students.
//! Answers within 10^-5 of the actual answer will be accepted.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution;

/// Custom struct to represent a class with its current state and potential gain
#[derive(Debug)]
struct ClassGain {
    gain: f64,
    pass: i32,
    total: i32,
}

impl PartialEq for ClassGain {
    fn eq(&self, other: &Self) -> bool {
        self.gain == other.gain
    }
}

impl Eq for ClassGain {}

impl PartialOrd for ClassGain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // For max heap, we want higher gains first
        self.gain.partial_cmp(&other.gain)
    }
}

impl Ord for ClassGain {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl ClassGain {
    /// Calculate the gain in pass ratio when adding one student to this class
    fn new(pass: i32, total: i32) -> Self {
        let gain = (total - pass) as f64 / (total as f64 * (total + 1) as f64);
        ClassGain { gain, pass, total }
    }

    /// Add one student to this class and return the updated ClassGain
    fn add_student(self) -> Self {
        ClassGain::new(self.pass + 1, self.total + 1)
    }

    /// Get the current pass ratio
    fn pass_ratio(&self) -> f64 {
        self.pass as f64 / self.total as f64
    }
}

impl Solution {
    /// Find the maximum average pass ratio after optimally assigning extra students
    ///
    /// Time Complexity: O((n + k) * log n) where n = classes.length, k = extraStudents
    /// Space Complexity: O(n) for the priority queue
    ///
    /// The algorithm uses a greedy approach with a max heap:
    /// 1. Calculate the potential gain for each class if we add one student
    /// 2. Always assign the next student to the class with maximum gain
    /// 3. Update the gain and repeat until all extra students are assigned
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        // Create a max heap of classes based on their potential gain
        let mut heap: BinaryHeap<ClassGain> = classes
            .into_iter()
            .map(|class| ClassGain::new(class[0], class[1]))
            .collect();

        // Assign each extra student to the class with maximum gain
        for _ in 0..extra_students {
            if let Some(best_class) = heap.pop() {
                // Add one student to the best class and put it back in the heap
                heap.push(best_class.add_student());
            }
        }

        // Calculate the average pass ratio
        let total_ratio: f64 = heap.iter().map(|class| class.pass_ratio()).sum();
        total_ratio / heap.len() as f64
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2, 0.78333),
        (
            vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]],
            4,
            0.53485,
        ),
        (vec![vec![1, 2]], 1, 2.0 / 3.0),
        (vec![vec![5, 5]], 1, 1.0),
    ];

    for (i, (classes, extra_students, expected)) in test_cases.iter().enumerate() {
        println!(
            "Example {}: classes = {:?}, extraStudents = {}",
            i + 1,
            classes,
            extra_students
        );
        let result = Solution::max_average_ratio(classes.clone(), *extra_students);
        println!("Output: {:.5}", result);
        println!("Expected: {:.5}", expected);
        println!("Passed: {}", (result - expected).abs() < 1e-5);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
        let extra_students = 2;
        let result = Solution::max_average_ratio(classes, extra_students);
        assert!((result - 0.78333).abs() < 1e-5);
    }

    #[test]
    fn test_example2() {
        let classes = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
        let extra_students = 4;
        let result = Solution::max_average_ratio(classes, extra_students);
        assert!((result - 0.53485).abs() < 1e-5);
    }

    #[test]
    fn test_single_class() {
        let classes = vec![vec![1, 2]];
        let extra_students = 1;
        let result = Solution::max_average_ratio(classes, extra_students);
        assert!((result - (2.0 / 3.0)).abs() < 1e-5);
    }

    #[test]
    fn test_perfect_class() {
        let classes = vec![vec![5, 5]];
        let extra_students = 1;
        let result = Solution::max_average_ratio(classes, extra_students);
        assert!((result - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_no_extra_students() {
        let classes = vec![vec![1, 2], vec![3, 5]];
        let extra_students = 0;
        let result = Solution::max_average_ratio(classes, extra_students);
        let expected = (0.5 + 0.6) / 2.0;
        assert!((result - expected).abs() < 1e-5);
    }

    #[test]
    fn test_multiple_classes_equal_gain() {
        let classes = vec![vec![1, 3], vec![1, 3]];
        let extra_students = 2;
        let result = Solution::max_average_ratio(classes, extra_students);
        // Each class gets one extra student: (2/4 + 2/4) / 2 = 0.5
        assert!((result - 0.5).abs() < 1e-5);
    }
}
