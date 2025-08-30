impl Solution {
    pub fn min_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut not_zero = nums.iter().filter(|&num| *num != 0).count();
        if not_zero == 0 {
            return 0;
        }
        for (i, query) in queries.iter().enumerate() {
            for j in query[0] as usize..=query[1] as usize {
                if nums[j] == 0 {
                    continue;
                }
                if nums[j] <= query[2] {
                    nums[j] = 0;
                    not_zero -= 1;
                    if not_zero == 0 {
                        return i as i32 + 1;
                    }
                } else {
                    nums[j] -= query[2]
                }
            }
        }
        -1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::min_zero_array(
                vec![2, 0, 2],
                vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            -1,
            Solution::min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]])
        );
    }
}
