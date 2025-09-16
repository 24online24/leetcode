impl Solution {
    fn greatest_common_divisor(mut x: i32, mut y: i32) -> i32 {
        if y > x {
            (x, y) = (y, x);
        }
        while y > 0 {
            (x, y) = (y, x % y);
        }
        x
    }

    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return nums;
        }
        let mut result = vec![];
        nums.into_iter().for_each(|mut num| {
            while let Some(&top) = result.last() {
                let gcd = Self::greatest_common_divisor(num, top);
                if gcd > 1 {
                    num = num / gcd * top;
                    result.pop();
                } else {
                    break;
                }
            }
            result.push(num);
        });
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn gcd() {
        assert_eq!(6, Solution::greatest_common_divisor(48, 18));
    }

    #[test]
    fn example_1() {
        assert_eq!(
            vec![12, 7, 6],
            Solution::replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2])
        );
    }

    #[test]
    fn example_17() {
        assert_eq!(vec![48757], Solution::replace_non_coprimes(vec![48757]));
    }

    #[test]
    fn example_23() {
        assert_eq!(
            vec![5687, 1887, 5],
            Solution::replace_non_coprimes(vec![517, 11, 121, 517, 3, 51, 3, 1887, 5])
        );
    }

    #[test]
    fn example_33() {
        assert_eq!(
            vec![31, 97561],
            Solution::replace_non_coprimes(vec![
                31, 97561, 97561, 97561, 97561, 97561, 97561, 97561, 97561
            ])
        );
    }

    #[test]
    fn example_57() {
        assert_eq!(
            vec![9901, 1927],
            Solution::replace_non_coprimes(vec![9901, 41, 41, 1927])
        );
    }

    #[test]
    fn example_59() {
        assert_eq!(
            vec![2009, 20677, 825],
            Solution::replace_non_coprimes(vec![287, 41, 49, 287, 899, 23, 23, 20677, 5, 825])
        );
    }
}
