impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut appearances = vec![false; 10_i32.pow(5) as usize + 1];
        nums.iter().for_each(|&num| {
            if appearances[num as usize] {
                result.push(num);
            } else {
                appearances[num as usize] = true;
            }
        });
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
