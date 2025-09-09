impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        let n = nums.len();
        if n < 2 {
            return;
        }
        while i < n && j < n {
            while i < n && nums[i] == 0 {
                i += 1;
            }
            while j < n && nums[j] != 0 {
                j += 1;
            }
            if i < n && j < n {
                if j < i {
                    nums[j] = nums[i];
                    nums[i] = 0;
                }
                i += 1;
                j += 1;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut input = vec![1, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(vec![1, 0], input);
    }
}
