impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut occurances = [0, 0, 0];
        for num in &mut *nums {
            occurances[*num as usize] += 1;
        }
        let mut i = 0;
        for color in 0..occurances.len() {
            while occurances[color] > 0 {
                nums[i] = color as i32;
                occurances[color] -= 1;
                i += 1;
            }
        }
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
