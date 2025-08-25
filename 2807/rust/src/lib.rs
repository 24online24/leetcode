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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        while let Some(mut node) = head {
            if let Some(next) = node.next {
                node.next = Some(Box::new(ListNode {
                    val: node.val + next.val,
                    next: next.next,
                }));
                node = next.next;
            }
        }
        head
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::ListNode;

    #[test]
    fn example_1() {
        let head = ListNode {
            val: 18,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 10,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        };
    }
}
