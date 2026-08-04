impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let nums = nums.iter();
        let minimum = nums.clone().cloned().min().unwrap();
        let maximum = nums.clone().max().unwrap();
        let mut appearance = vec![false; (maximum - minimum) as usize + 1];
        nums.for_each(|num| appearance[(*num - minimum) as usize] = true);
        appearance
            .into_iter()
            .enumerate()
            .filter_map(|(idx, appears)| (!appears).then_some(idx as i32 + minimum))
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![3], Solution::find_missing_elements(vec![1, 4, 2, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_missing_elements(vec![7, 8, 6, 9])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![2, 3, 4], Solution::find_missing_elements(vec![5, 1]));
    }
}
