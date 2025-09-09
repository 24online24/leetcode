use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();
        strs.into_iter().for_each(|str| {
            let mut sorted = str.chars().collect::<Vec<_>>();
            sorted.sort();
            anagrams
                .entry(sorted)
                .and_modify(|anagram_array: &mut Vec<String>| anagram_array.push(str.clone()))
                .or_insert(vec![str]);
        });
        anagrams.into_values().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
