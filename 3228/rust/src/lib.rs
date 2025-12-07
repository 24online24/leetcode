impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let bits = s.as_bytes();
        let mut last_0 = s.len() - 1;
        let mut last_1;
        let mut operations = 0;
        while last_0 > 0 && bits[last_0] == b'1' {
            last_0 -= 1;
        }
        loop {
            last_1 = last_0 - 1;
            while last_1 > 0 && bits[last_1] == b'0' {
                last_1 -= 1;
            }
            operations += last_0 - last_1;
            if last_1 == 0 {
                break;
            }
            last_0 = last_1;
        }
        operations as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::max_operations("1001101".to_string()))
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::max_operations("00111".to_string()))
    }
}
