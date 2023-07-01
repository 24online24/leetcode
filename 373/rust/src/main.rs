// use std::collections::HashMap;
use std::cmp::min;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // let mut pairs = HashMap::<(i32, i32), i32>::new();
        let mut pairs_with_sum = Vec::new();
        let mut answer = Vec::new();
        for element1 in &nums1 {
            for element2 in &nums2 {
                // pairs.insert((*element1, *element2), element1 );
                pairs_with_sum.push((vec![*element1, *element2], element1 + element2));
            }
        }
        println!("{:?}", pairs_with_sum);
        pairs_with_sum.sort_by(|a, b| (a.1).cmp(&b.1));
        println!("{:?}", pairs_with_sum);
        for i in 0..min(k as usize, pairs_with_sum.len()) {
            answer.push(pairs_with_sum[i].clone().0);
        }
        answer
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
    );
    println!(
        "{:?}",
        Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2)
    );
}
