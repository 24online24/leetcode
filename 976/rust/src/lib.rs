impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut i = nums.len() - 1;
        while i > 1 && nums[i] >= nums[i - 1] + nums[i - 2] {
            i -= 1;
        }
        if i > 1 {
            nums[i] + nums[i - 1] + nums[i - 2]
        } else {
            0
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::largest_perimeter(vec![2, 1, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::largest_perimeter(vec![1, 2, 1, 10]));
    }
}
