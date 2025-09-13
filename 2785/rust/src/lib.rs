use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    const VOWELS: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

    pub fn sort_vowels(s: String) -> String {
        let mut ordered_vowels_in_word = BinaryHeap::new();
        s.chars().for_each(|chr| {
            if Self::VOWELS.contains(&chr) {
                ordered_vowels_in_word.push(Reverse(chr));
            }
        });
        s.chars()
            .map(|chr| {
                if !Self::VOWELS.contains(&chr) {
                    chr
                } else {
                    ordered_vowels_in_word.pop().unwrap().0
                }
            })
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!("lEOtcede", Solution::sort_vowels("lEetcOde".to_string()));
    }
}
