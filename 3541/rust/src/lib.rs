impl Solution {
    const VOWELS: &[u8] = "aeiou".as_bytes();

    pub fn max_freq_sum(s: String) -> i32 {
        let mut appearances = [0; 26];
        s.bytes()
            .for_each(|chr| appearances[(chr - b'a') as usize] += 1);

        let mut max_app_vowel = 0;
        let mut max_app_cons = 0;
        for (byte, &appearance_count) in appearances.iter().enumerate() {
            if Self::VOWELS.contains(&(byte as u8 + b'a')) {
                if appearance_count > max_app_vowel {
                    max_app_vowel = appearance_count;
                }
            } else if appearance_count > max_app_cons {
                max_app_cons = appearance_count;
            }
        }
        max_app_vowel + max_app_cons
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(6, Solution::max_freq_sum("successes".to_owned()))
    }
}
