use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut x = map.into_iter().map(|(x, y)| (y, x)).collect::<Vec<(i32, i32)>>();
        x.sort();
        x.into_iter().map(|key_freq| key_freq.0).collect()
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
