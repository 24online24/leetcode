impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum MatchType {
            None,
            Vowel,
            Case,
        }

        let vowels: Vec<u8> = "aeiou".into();

        queries
            .into_iter()
            .map(|query| {
                let mut best_match = "".to_owned();
                let mut best_match_type = MatchType::None;
                let query_len = query.len();
                for word in &wordlist {
                    if query_len != word.len() {
                        continue;
                    }
                    if query == *word {
                        best_match = word.clone();
                        break;
                    } else {
                        let query_lowercase = query.to_ascii_lowercase();
                        let word_lowercase = word.to_ascii_lowercase();
                        if best_match_type < MatchType::Case && query_lowercase == *word_lowercase {
                            best_match = word.clone();
                            best_match_type = MatchType::Case;
                        } else if best_match_type == MatchType::None {
                            let query_bytes = query_lowercase.as_bytes();
                            let word_bytes = word_lowercase.as_bytes();
                            let mut i = 0;
                            while i < query_len {
                                if query_bytes[i] != word_bytes[i]
                                    && (!vowels.contains(&query_bytes[i])
                                        || !vowels.contains(&word_bytes[i]))
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i == query_len {
                                best_match = word.clone();
                                best_match_type = MatchType::Vowel;
                            }
                        }
                    }
                }
                best_match
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
                vec![
                    "KiTe".to_string(),
                    "kite".to_string(),
                    "hare".to_string(),
                    "Hare".to_string()
                ],
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
