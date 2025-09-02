//! LeetCode Solutions Library
//!
//! This library contains common utilities and helper functions
//! that can be shared across different LeetCode solutions.

/// Common data structures used in LeetCode problems

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// Create a linked list from a vector
    pub fn from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in vals.into_iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    /// Convert linked list to vector
    pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

/// Helper functions for common operations

/// Print a vector in a formatted way
pub fn print_vec<T: std::fmt::Debug>(vec: &[T]) {
    println!("{:?}", vec);
}

/// Print execution time of a function
pub fn time_it<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listnode_from_vec() {
        let vec = vec![1, 2, 3];
        let list = ListNode::from_vec(vec.clone());
        let result = ListNode::to_vec(list);
        assert_eq!(result, vec);
    }

    #[test]
    fn test_empty_listnode() {
        let list = ListNode::from_vec(vec![]);
        assert_eq!(list, None);
        let result = ListNode::to_vec(list);
        assert_eq!(result, vec![]);
    }
}
