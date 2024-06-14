use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.clone() {
            let mut root = root.borrow_mut();
            if root.left.is_some() {
                root.left = Solution::remove_leaf_nodes(root.left.clone(), target);
            }
            if root.right.is_some() {
                root.right = Solution::remove_leaf_nodes(root.right.clone(), target);
            }
            if root.left.is_none() && root.right.is_none() && root.val == target {
                return None;
            }
        }
        root
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
