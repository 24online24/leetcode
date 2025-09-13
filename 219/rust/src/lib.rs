impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        for (i, num_i) in nums.iter().enumerate() {
            for num_j in nums.iter().skip(i + 1).take(k as usize) {
                if num_j == num_i {
                    return true;
                }
            }
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3))
    }
}
