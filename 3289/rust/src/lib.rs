use std::collections::HashSet;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(2);
        let mut appeared = HashSet::new();
        for num in nums {
            if !appeared.insert(num) {
                result.push(num);
            }
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
