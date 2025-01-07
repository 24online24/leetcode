impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut nice = 0;
        let k_usize = k as usize;
        let mut odd = nums[0..k_usize].iter().fold(0, |acc, num| acc + num % 2);
        for i in k_usize..nums.len() {
            odd = odd + nums[i] % 2 - nums[i - 1] % 2;
            if odd == k {
                nice += 1;
            }
        }
        nice
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
