impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        let mut increment = 0;
        nums.sort_unstable();
        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            if diff <= 0 {
                nums[i] += -diff + 1;
                increment += -diff + 1;
            }
        }
        increment
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
