use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        fn unvowel(word: &str) -> String {
            word.to_ascii_lowercase()
                .chars()
                .map(|chr| if "aeiou".contains(chr) { '_' } else { chr })
                .collect()
        }

        let mut exact_words: HashSet<&String> = HashSet::new();
        let mut lower_case_words: HashMap<String, &String> = HashMap::new();
        let mut unvoweled_words: HashMap<String, &String> = HashMap::new();
        wordlist.iter().for_each(|word| {
            exact_words.insert(word);
            lower_case_words
                .entry(word.to_ascii_lowercase())
                .or_insert(word);
            unvoweled_words.entry(unvowel(word)).or_insert(word);
        });

        queries
            .into_iter()
            .map(|query| {
                if exact_words.contains(&query) {
                    query
                } else if let Some(&word) = lower_case_words.get(&query.to_ascii_lowercase()) {
                    word.clone()
                } else if let Some(&word) = unvoweled_words.get(&unvowel(&query)) {
                    word.clone()
                } else {
                    "".to_string()
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
    fn example_1() {
        assert_eq!(
            vec!["KiTe", "hare", "KiTe", "KiTe"],
            Solution::spellchecker(
                vec![
                    "KiTe".to_string(),
                    "kite".to_string(),
                    "hare".to_string(),
                    "Hare".to_string()
                ],
                vec![
                    "Kite".to_string(),
                    "HARE".to_string(),
                    "keti".to_string(),
                    "keto".to_string()
                ]
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec!["KiTe"],
            Solution::spellchecker(
                vec!["KiTe".to_string(), "kite".to_string(),],
                vec!["Kite".to_string()]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec!["yellow"],
            Solution::spellchecker(vec!["yellow".to_string()], vec!["YellOw".to_string()])
        );
    }
}
