impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut appearance = vec![false; nums.len() + 1];
        nums.into_iter().for_each(|num| {
            appearance[num as usize] = true;
        });
        appearance
            .into_iter()
            .skip(1)
            .enumerate()
            .filter(|(_, appears)| !appears)
            .map(|(num, _)| num as i32 + 1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
