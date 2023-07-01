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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = root.clone().unwrap().borrow().val;

        let mut queue = vec![root];
        let mut max_level = 1;
        let mut level = 1;

        while !queue.is_empty() {
            let mut next_level: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
            let mut sum = 0;
            level += 1;
            let mut next_level_exists = false;

            for node_option in queue {
                let node_ref = node_option.unwrap();

                let actual_node = node_ref.borrow();

                if let Some(left_child_ref) = actual_node.left.clone() {
                    sum += left_child_ref.borrow().val;
                    next_level_exists = true;
                    next_level.push(Some(left_child_ref));
                }

                if let Some(right_child_ref) = actual_node.right.clone() {
                    sum += right_child_ref.borrow().val;
                    next_level_exists = true;
                    next_level.push(Some(right_child_ref));
                }
            }

            if next_level_exists {
                if sum > max_sum {
                    max_sum = sum;
                    max_level = level;
                }
            }

            queue = next_level;
        }

        max_level
    }
}

struct Solution;
fn main() {
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-8)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    };

    println!(
        "{}",
        Solution::max_level_sum(Some(Rc::new(RefCell::new(root))))
    );
}
