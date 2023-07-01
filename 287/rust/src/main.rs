struct Solution();

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut freq = vec![false; nums.len() + 1];
        for element in nums {
            if freq[element as usize] == true {
                return element;
            }
            freq[element as usize] = true;
        }
        0
    }
}

fn main() {
    // let solution = Solution();
    let nums1 = vec![1, 3, 4, 2, 2];
    let nums2 = vec![3, 1, 3, 4, 2];
    println!("{}", Solution::find_duplicate(nums1));
    println!("{}", Solution::find_duplicate(nums2));
}
