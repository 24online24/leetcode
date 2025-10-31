use std::collections::HashSet;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut appeared = HashSet::new();
        nums.into_iter()
            .filter(|num| !appeared.insert(*num))
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
