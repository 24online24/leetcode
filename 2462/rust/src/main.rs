use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut cost: i64 = 0;
        let candidates_usize = candidates as usize;
        let mut head_pointer = candidates_usize;

        let mut head_elements = BinaryHeap::from(
            (&costs[..candidates_usize])
                .iter()
                .map(|x| Reverse(x))
                .collect::<Vec<Reverse<&i32>>>(),
        );
        let length = costs.len();
        let mut tail_pointer = length;
        let mut tail_elements = BinaryHeap::new();
        if length == candidates_usize {
        } else {
            tail_pointer = max(length - candidates_usize - 1, candidates_usize - 1);
            tail_elements = BinaryHeap::from(
                (&costs[tail_pointer + 1..])
                    .iter()
                    .map(|x| Reverse(x))
                    .collect::<Vec<Reverse<&i32>>>(),
            );
        }

        for _ in 0..k {
            let mut head_cost = &Reverse(&100001);
            let mut tail_cost = &Reverse(&100001);
            if let Some(head_element_extracted) = head_elements.peek() {
                head_cost = head_element_extracted;
            }
            if let Some(tail_element_extracted) = tail_elements.peek() {
                tail_cost = tail_element_extracted;
            }
            if head_cost >= tail_cost {
                println!("Added head_cost");
                cost += *head_elements.pop().unwrap().0 as i64;
                if head_pointer <= tail_pointer && head_pointer < length {
                    head_elements.push(Reverse(&costs[head_pointer]));
                    head_pointer += 1;
                }
            } else {
                cost += *tail_elements.pop().unwrap().0 as i64;
                if head_pointer <= tail_pointer {
                    tail_elements.push(Reverse(&costs[tail_pointer]));
                    tail_pointer -= 1;
                }
            }
        }
        cost
    }
}
struct Solution;
fn main() {
    assert_eq!(
        Solution::total_cost([17, 12, 10, 2, 7, 2, 11, 20, 8].to_vec(), 3, 4),
        11
    );
    println!("================== TEST 2 ==================");
    assert_eq!(Solution::total_cost([1, 2, 4, 1].to_vec(), 3, 3), 4);
    println!("================== TEST 3 ==================");
    assert_eq!(
        Solution::total_cost(
            [
                69, 10, 63, 24, 1, 71, 55, 46, 4, 61, 78, 21, 85, 52, 83, 77, 42, 21, 73, 2, 80,
                99, 98, 89, 55, 94, 63, 50, 43, 62, 14
            ]
            .to_vec(),
            21,
            31
        ),
        -1
    );
}
