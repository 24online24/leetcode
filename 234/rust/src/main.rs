// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut v: Vec<i32> = vec![];
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }
        let length = v.len() - 1;
        if length == 0 {
            return true;
        }
        for i in 0..=(length - 1) / 2 {
            if v[i] != v[length - i] {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::is_palindrome(Some(Box::new(ListNode::new(2)))),
        true
    );
}
