impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        for set_size in 1..nums.len() + 1 {
            for starting_position in 0..=nums.len() - set_size {
                let mut current = Vec::new();
                for i in starting_position..starting_position + set_size {
                    current.push(nums[i])
                }
                result.push(current);
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
