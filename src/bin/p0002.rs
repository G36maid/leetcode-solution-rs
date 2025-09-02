//! Problem 2: Add Two Numbers
//!
//! You are given two non-empty linked lists representing two non-negative integers.
//! The digits are stored in reverse order, and each of their nodes contains a single digit.
//! Add the two numbers and return the sum as a linked list.
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! Example 1:
//! Input: l1 = [2,4,3], l2 = [5,6,4]
//! Output: [7,0,8]
//! Explanation: 342 + 465 = 807.
//!
//! Example 2:
//! Input: l1 = [0], l2 = [0]
//! Output: [0]
//!
//! Example 3:
//! Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//! Output: [8,9,9,9,0,0,0,1]

use leetcode::ListNode;

struct Solution;

impl Solution {
    /// Add two numbers represented as linked lists
    ///
    /// Algorithm: Simulate addition digit by digit with carry
    /// Time Complexity: O(max(m, n)) where m and n are lengths of the two lists
    /// Space Complexity: O(max(m, n)) for the result list
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut carry = 0;
        let mut p1 = l1;
        let mut p2 = l2;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let val1 = if let Some(node) = p1 {
                let val = node.val;
                p1 = node.next;
                val
            } else {
                0
            };

            let val2 = if let Some(node) = p2 {
                let val = node.val;
                p2 = node.next;
                val
            } else {
                0
            };

            let sum = val1 + val2 + carry;
            carry = sum / 10;

            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        dummy_head.next
    }

    /// Alternative recursive solution
    #[allow(dead_code)]
    pub fn add_two_numbers_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_recursive(l1, l2, 0)
    }

    /// Helper function for recursive solution
    fn add_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let val1 = l1.as_ref().map_or(0, |node| node.val);
        let val2 = l2.as_ref().map_or(0, |node| node.val);
        let sum = val1 + val2 + carry;

        let next1 = l1.and_then(|node| node.next);
        let next2 = l2.and_then(|node| node.next);

        Some(Box::new(ListNode {
            val: sum % 10,
            next: Self::add_recursive(next1, next2, sum / 10),
        }))
    }
}

fn main() {
    // Test cases
    let test_cases = vec![
        (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
        (vec![0], vec![0], vec![0]),
        (
            vec![9, 9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9],
            vec![8, 9, 9, 9, 0, 0, 0, 1],
        ),
    ];

    for (i, (l1_vec, l2_vec, expected_vec)) in test_cases.iter().enumerate() {
        println!("Example {}: ", i + 1);
        println!("Input: l1 = {:?}, l2 = {:?}", l1_vec, l2_vec);

        let l1 = ListNode::from_vec(l1_vec.clone());
        let l2 = ListNode::from_vec(l2_vec.clone());
        let result = Solution::add_two_numbers(l1, l2);
        let result_vec = ListNode::to_vec(result);

        println!("Output: {:?}", result_vec);
        println!("Expected: {:?}", expected_vec);
        println!("Passed: {}", result_vec == *expected_vec);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers_example1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_example2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_example3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_different_lengths() {
        let l1 = ListNode::from_vec(vec![2, 4, 3, 1]);
        let l2 = ListNode::from_vec(vec![5, 6]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![7, 0, 4, 1]);
    }

    #[test]
    fn test_add_two_numbers_with_carry() {
        let l1 = ListNode::from_vec(vec![5]);
        let l2 = ListNode::from_vec(vec![5]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0, 1]);
    }

    #[test]
    fn test_add_two_numbers_recursive() {
        let test_cases = vec![
            (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
            (vec![0], vec![0], vec![0]),
            (
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            ),
        ];

        for (l1_vec, l2_vec, expected) in test_cases {
            let l1 = ListNode::from_vec(l1_vec);
            let l2 = ListNode::from_vec(l2_vec);
            let result = Solution::add_two_numbers_recursive(l1, l2);
            assert_eq!(ListNode::to_vec(result), expected);
        }
    }

    #[test]
    fn test_both_solutions_consistency() {
        let test_cases = vec![
            (vec![2, 4, 3], vec![5, 6, 4]),
            (vec![0], vec![0]),
            (vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9]),
            (vec![1], vec![9, 9, 9]),
        ];

        for (l1_vec, l2_vec) in test_cases {
            let l1_1 = ListNode::from_vec(l1_vec.clone());
            let l2_1 = ListNode::from_vec(l2_vec.clone());
            let result1 = Solution::add_two_numbers(l1_1, l2_1);

            let l1_2 = ListNode::from_vec(l1_vec);
            let l2_2 = ListNode::from_vec(l2_vec);
            let result2 = Solution::add_two_numbers_recursive(l1_2, l2_2);

            assert_eq!(ListNode::to_vec(result1), ListNode::to_vec(result2));
        }
    }

    #[test]
    fn test_edge_cases() {
        // Single digit addition with carry
        let l1 = ListNode::from_vec(vec![9]);
        let l2 = ListNode::from_vec(vec![1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0, 1]);

        // Empty-like case (single zero)
        let l1 = ListNode::from_vec(vec![1]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![1]);
    }
}
