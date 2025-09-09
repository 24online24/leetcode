impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut longest = 1;
        let mut current = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                current += 1;
            } else {
                if current > longest {
                    longest = current;
                }
                current = 1;
            }
        }
        longest
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
