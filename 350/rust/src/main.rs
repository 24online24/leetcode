use std::{cmp::min, collections::HashMap};

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut freq1 = HashMap::new();
        let mut freq2 = HashMap::new();
        for num in nums1 {
            *freq1.entry(num).or_insert(0) += 1;
        }
        for num in nums2 {
            *freq2.entry(num).or_insert(0) += 1;
        }

        let (smaller, larger) = if freq1.len() <= freq2.len() {
            (freq1, freq2)
        } else {
            (freq2, freq1)
        };

        for (num, f1) in smaller.iter() {
            if let Some(f2) = larger.get(num) {
                for _ in 0..*min(f1, f2) {
                    result.push(*num);
                }
            }
        }

        result
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
