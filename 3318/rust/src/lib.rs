use std::collections::BinaryHeap;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        fn x_sum(nums: &[i32], x: i32) -> i32 {
            let mut frequencies = [0; 51];
            for &num in nums {
                frequencies[num as usize] += 1;
            }

            let mut heap = BinaryHeap::new();
            for (num, freq) in frequencies.into_iter().enumerate() {
                if freq > 0 {
                    heap.push((freq, num));
                }
            }

            let mut sum = 0;
            let mut i = 0;
            while let Some((frequency, num)) = heap.pop()
                && i < x
            {
                sum += frequency * num as i32;
                i += 1;
            }
            sum
        }
        nums.windows(k as usize)
            .map(|subarray| x_sum(subarray, x))
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example9() {
        assert_eq!(vec![13], Solution::find_x_sum(vec![9, 2, 2], 2, 2));
    }
}
