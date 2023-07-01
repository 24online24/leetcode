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

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![root];
        let mut values: Vec<i32> = vec![];
        let mut min_dif = 10_i32.pow(5) + 1;

        while let Some(node) = stack.pop() {
            if let Some(actual_node) = node {
                let node_reference = actual_node.borrow();
                values.push(node_reference.val);

                // if let Some(left_child) = &node_reference.left {
                //     stack.push(Some(left_child.clone()));
                // }

                // if let Some(right_child) = node_reference.right.as_ref() {
                //     stack.push(Some(right_child.clone()));
                // }

                if node_reference.left.is_some() {
                    stack.push(node_reference.left.clone());
                }

                if node_reference.right.is_some() {
                    stack.push(node_reference.right.clone())
                }
            }
        }

        values.sort();
        for i in 0..values.len() - 1 {
            let dif = values[i + 1] - values[i];
            if dif < min_dif {
                min_dif = dif;
            }
        }
        min_dif
    }
}

struct Solution;
fn main() {
    let node1 = TreeNode {
        val: 1,
        left: None,
        right: None,
    };

    let node3 = TreeNode {
        val: 3,
        left: None,
        right: None,
    };

    let node2 = TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(node1))),
        right: Some(Rc::new(RefCell::new(node3))),
    };

    let node6 = TreeNode {
        val: 6,
        left: None,
        right: None,
    };

    let node4 = TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(node2))),
        right: Some(Rc::new(RefCell::new(node6))),
    };

    println!(
        "{}",
        Solution::get_minimum_difference(Some(Rc::new(RefCell::new(node4))))
    );
}
