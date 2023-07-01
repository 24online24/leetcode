impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut filtered_nums: Vec<i32> = nums.into_iter().filter(|&element| element < k).collect();
        if filtered_nums.len() == 0 {
            return 0;
        }

        filtered_nums.sort();

        let mut operations = 0;
        let mut left_index = 0;
        let mut right_index = filtered_nums.len() - 1;

        let mut sum;
        while left_index < right_index {
            sum = filtered_nums[left_index] + filtered_nums[right_index];
            if sum < k {
                left_index += 1;
            } else if sum > k {
                right_index -= 1;
            } else if sum == k {
                operations += 1;
                left_index += 1;
                right_index -= 1;
            }
        }
        operations
    }
}

struct Solution;

fn main() {
    // println!("{}", Solution::max_operations(vec![1, 2, 3, 4], 5));
    // println!("{}", Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
    // println!("{}", Solution::max_operations(vec![4, 4, 4, 4, 1], 5));
    // println!("{}", Solution::max_operations(vec![1, 4, 4, 4, 4], 5));
    println!(
        "{}",
        Solution::max_operations(
            vec![
                48, 87, 82, 57, 69, 63, 33, 58, 71, 7, 44, 52, 81, 34, 68, 24, 20, 10, 3, 82, 59,
                20, 59, 10, 66, 62, 51, 57, 3, 24, 10, 84, 3, 16, 77, 27, 90, 5, 35, 56, 27, 82,
                21, 14, 20, 31, 23, 23, 15, 87, 73, 13, 8, 29, 27, 74, 80, 61, 77, 19, 10, 20, 4,
                81, 74, 11, 63, 72, 77, 78, 32, 90, 77, 32, 85, 78, 48, 38, 63, 82, 69, 59, 85, 82,
                43, 54, 44, 32, 71, 5, 69, 5, 42, 65, 61, 34, 13, 89, 76, 71, 77, 37, 6, 50, 53,
                13, 30, 5, 86, 5, 88, 53, 23, 53, 38, 29, 83, 70, 36, 74, 68, 37, 15, 78, 17, 85
            ],
            12
        )
    );
}
