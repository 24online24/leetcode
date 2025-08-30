impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;
        let mut prefix = String::new();
        let words_as_bytes: Vec<Vec<char>> =
            strs.into_iter().map(|str| str.chars().collect()).collect();
        let first_word = words_as_bytes[0].clone();
        loop {
            if i >= first_word.len() {
                return prefix;
            }
            let expected = first_word[i];
            for word in words_as_bytes.iter().skip(1) {
                if i >= word.len() || word[i] != expected {
                    return prefix;
                }
            }
            prefix.push(expected);
            i += 1;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            "fl",
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!("", Solution::longest_common_prefix(vec!["".to_string()]));
    }
}
