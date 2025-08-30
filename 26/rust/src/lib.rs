impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut i = n - 1;
        while i > 0 {
            while n > i && nums[i] == nums[i - 1] {
                for j in i..n - 1 {
                    nums[j] = nums[j + 1];
                }
                n -= 1;
            }
            i -= 1;
        }
        n as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
