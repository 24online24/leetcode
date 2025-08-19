impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut ways = 0;
        for one_step in (n % 2..=n).step_by(2) {
            let two_step = (n - one_step) / 2;
            let bigger;
            let smaller;
            if one_step >= two_step {
                bigger = one_step as u128;
                smaller = two_step as u128;
            } else {
                bigger = two_step as u128;
                smaller = one_step as u128;
            }
            let mut numerator = 1;
            for i in bigger + 1..=bigger + smaller {
                numerator *= i;
            }
            let mut denominator = 1;
            for i in 2..=smaller {
                denominator *= i;
            }
            ways += numerator / denominator;
        }
        ways as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(5, Solution::climb_stairs(4));
        assert_eq!(14930352, Solution::climb_stairs(35));
    }
}
