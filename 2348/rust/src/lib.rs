impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut subarrays = 0;
        let mut zero_count = 0;
        nums.iter().for_each(|&num| {
            if num == 0 {
                zero_count += 1;
            } else {
                subarrays += zero_count * (zero_count + 1) / 2;
                zero_count = 0;
            }
        });
        subarrays + zero_count * (zero_count + 1) / 2
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])
        );
        assert_eq!(9, Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]));
        assert_eq!(0, Solution::zero_filled_subarray(vec![2, 10, 2019]));
    }
}
