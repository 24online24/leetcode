impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut total_time = 0;
        let mut time_sum = needed_time[0];
        let mut longest_time = needed_time[0];
        let bytes = colors.as_bytes();
        for idx in 1..bytes.len() {
            let needed = needed_time[idx];
            if bytes[idx] == bytes[idx - 1] {
                time_sum += needed;
                if needed > longest_time {
                    longest_time = needed;
                }
            } else {
                total_time += time_sum - longest_time;
                time_sum = needed;
                longest_time = needed;
            }
        }
        total_time + time_sum - longest_time
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            3,
            Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(0, Solution::min_cost("abc".to_string(), vec![1, 2, 3]));
    }

    #[test]
    fn ex3() {
        assert_eq!(
            2,
            Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1])
        );
    }
}
