impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i] % 2 == nums[i - 1] % 2 {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::is_array_special(vec![1]));
        assert!(Solution::is_array_special(vec![2, 1, 4]));
        assert!(!Solution::is_array_special(vec![4, 3, 1, 6]));
    }
}
