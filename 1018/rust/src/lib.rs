impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut answer = Vec::with_capacity(nums.len());

        let mut reminder = 0;
        for num in nums {
            reminder = (reminder * 2 + num) % 5;
            answer.push(reminder == 0);
        }

        answer
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![true, false, false, false, true, false],
            Solution::prefixes_div_by5(vec![0, 1, 1, 1, 1, 1])
        );
    }
}
