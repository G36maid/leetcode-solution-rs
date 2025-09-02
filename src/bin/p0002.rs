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

fn main() {
    // Test case 1: [2,4,3] + [5,6,4] = [7,0,8]
    let l1 = ListNode::from_vec(vec![2, 4, 3]);
    let l2 = ListNode::from_vec(vec![5, 6, 4]);
    let result = add_two_numbers(l1, l2);
    println!("Test 1: {:?}", ListNode::to_vec(result));

    // Test case 2: [0] + [0] = [0]
    let l1 = ListNode::from_vec(vec![0]);
    let l2 = ListNode::from_vec(vec![0]);
    let result = add_two_numbers(l1, l2);
    println!("Test 2: {:?}", ListNode::to_vec(result));

    // Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
    let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
    let result = add_two_numbers(l1, l2);
    println!("Test 3: {:?}", ListNode::to_vec(result));
}

/// Add two numbers represented as linked lists
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers_example1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_example2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_example3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_different_lengths() {
        let l1 = ListNode::from_vec(vec![2, 4, 3, 1]);
        let l2 = ListNode::from_vec(vec![5, 6]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![7, 0, 4, 1]);
    }

    #[test]
    fn test_add_two_numbers_with_carry() {
        let l1 = ListNode::from_vec(vec![5]);
        let l2 = ListNode::from_vec(vec![5]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0, 1]);
    }
}
