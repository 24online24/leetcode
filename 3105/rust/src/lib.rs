impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 1;
        let n = nums.len();
        while i < n && nums[i] == nums[0] {
            i += 1;
        }
        if i < n {
            let mut longest = 2;
            let mut current = 2;
            let mut ascending = nums[i] > nums[0];
            i += 1;
            while i < n {
                if nums[i] != nums[i - 1] {
                    if (nums[i] > nums[i - 1]) == ascending {
                        current += 1;
                    } else {
                        if current > longest {
                            longest = current;
                        }
                        current = 2;
                        ascending = !ascending;
                    }
                } else {
                    if current > longest {
                        longest = current;
                    }
                    current = 1;
                }
                i += 1;
            }
            if current > longest {
                longest = current;
            }
            return longest;
        }
        1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(2, Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]));
        assert_eq!(3, Solution::longest_monotonic_subarray(vec![3, 2, 1]));
    }
}
