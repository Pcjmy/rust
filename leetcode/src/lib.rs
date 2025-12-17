// Common utilities for LeetCode problems

use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[allow(dead_code)]
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        if vec.is_empty() {
            return None;
        }

        let mut nodes: Vec<Box<ListNode>> = vec.into_iter()
            .map(|val| Box::new(ListNode::new(val)))
            .collect();

        for i in 0..nodes.len() - 1 {
            nodes[i].next = Some(nodes[i + 1].clone());
        }

        Some(nodes[0].clone())
    }

    #[allow(dead_code)]
    pub fn to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(n) = node {
            result.push(n.val);
            node = n.next;
        }
        result
    }
}

// Definition for a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[allow(dead_code)]
    pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() || vec[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while !queue.is_empty() && i < vec.len() {
            let node = queue.pop_front().unwrap();

            // Left child
            if i < vec.len() {
                if let Some(val) = vec[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
            }
            i += 1;

            // Right child
            if i < vec.len() {
                if let Some(val) = vec[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
            }
            i += 1;
        }

        Some(root)
    }
}

// Helper macro for testing
#[macro_export]
macro_rules! vec_of_option {
    ($($x:expr),*) => {
        vec![$(Some($x)),*]
    };
    (@none $($x:expr),*) => {
        vec![$($x),*]
    };
}

// Helper function to create vector with None values
pub fn vec_with_nones(items: Vec<Option<i32>>) -> Vec<Option<i32>> {
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_node_from_to_vec() {
        let vec1 = vec![1, 2, 3, 4, 5];
        let list = ListNode::from_vec(vec1.clone());
        let result = ListNode::to_vec(list);
        assert_eq!(result, vec1);
    }

    #[test]
    fn test_tree_node_from_vec() {
        let tree_vec = vec![Some(1), Some(2), Some(3), None, Some(4)];
        let tree = TreeNode::from_vec(tree_vec);
        assert!(tree.is_some());
        // More comprehensive tests could be added based on specific needs
    }
}