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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut modify = head.unwrap().next; // Initialize modify with the first non-zero node
        let mut next_sum = modify.as_mut(); // Mutable reference to iterate through the list

        while let Some(mut node) = next_sum {
            let mut sum = 0;

            // Find the sum of all nodes until we encounter a 0
            while node.val != 0 {
                sum += node.val;
                next_sum = node.next.as_mut();
                if let Some(n) = next_sum {
                    node = n;
                } else {
                    break;
                }
            }

            // Assign the sum to the current node's value
            modify.as_mut().unwrap().val = sum;
            // Move modify to the next node in the list
            modify = modify.unwrap().next;
            next_sum = modify.as_mut();
        }

        head.unwrap().next
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
