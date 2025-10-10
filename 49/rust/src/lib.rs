use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();
        strs.into_iter().for_each(|str| {
            anagrams
                .entry(str.bytes().fold([0; 26], |mut acc, x| {
                    acc[(x - b'a') as usize] += 1;
                    acc
                }))
                .or_insert_with(Vec::new)
                .push(str);
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
