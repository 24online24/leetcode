use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(value) => {
                let value_borrowed = value.borrow();
                match value_borrowed.val {
                    0 => false,
                    1 => true,
                    2 => {
                        Solution::evaluate_tree(value_borrowed.left.clone())
                            || Solution::evaluate_tree(value_borrowed.right.clone())
                    }
                    3 => {
                        Solution::evaluate_tree(value_borrowed.left.clone())
                            && Solution::evaluate_tree(value_borrowed.right.clone())
                    }
                    _ => false,
                }
            }
            None => false,
        }
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
