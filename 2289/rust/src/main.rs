impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let mut removed_something = true;
        let mut current_nums = nums;
        while removed_something {
            let mut next_nums = vec![current_nums[0]];
            removed_something = false;
            for i in 1..current_nums.len() {
                if current_nums[i - 1] <= current_nums[i] {
                    next_nums.push(current_nums[i]);
                } else {
                    removed_something = true;
                }
            }
            if removed_something {
                steps += 1;
            }
            current_nums = next_nums;
        }
        steps
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]),
        3
    );
    assert_eq!(Solution::total_steps(vec![4, 5, 7, 7, 13]), 0);
}
