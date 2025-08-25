impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        nums.extend(nums.clone());
        nums
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
