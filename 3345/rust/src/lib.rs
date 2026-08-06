impl Solution {
    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        fn prod(mut x: i32) -> i32 {
            if x == 0 {
                return 0;
            }
            let mut prod = 1;
            while x > 0 {
                prod *= x % 10;
                x /= 10;
            }
            prod
        }

        while prod(n) % t != 0 {
            n += 1;
        }
        n
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(10, Solution::smallest_number(10, 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(16, Solution::smallest_number(15, 3));
    }
}
