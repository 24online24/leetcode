impl Solution {
    fn binary_search(nums: Vec<i32>, target: i32) -> Option<usize> {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        println!("{:?} {}", nums, target);
        while l <= r {
            let m = ((l + r) / 2) as usize;
            if nums[m] < target {
                l = m as i32 + 1;
            } else if nums[m] > target {
                r = m as i32 - 1;
            } else {
                return Some(m);
            }
        }
        None
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (index, element) in nums.iter().enumerate() {
            let complement_index =
                Self::binary_search(nums[index + 1..].to_vec(), target - element);
            if let Some(found_complement_index) = complement_index {
                return vec![index as i32, (index + 1 + found_complement_index) as i32];
            }
        }
        vec![]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
}
