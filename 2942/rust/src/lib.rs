impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .into_iter()
            .enumerate()
            .filter(|(_, word)| word.contains(x))
            .map(|(idx, _)| idx as i32)
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![0, 1],
            Solution::find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e')
        );
    }
}
