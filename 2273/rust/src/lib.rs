impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        fn get_character_frequencies(word: String) -> [i32; 26] {
            word.bytes().fold([0; 26], |mut acc, x| {
                acc[(x - b'a') as usize] += 1;
                acc
            })
        }

        let mut result = Vec::with_capacity(words.len());
        let mut words_iter = words.into_iter();
        let mut characters_last;

        let first = words_iter.next().unwrap();
        result.push(first.clone());
        characters_last = get_character_frequencies(first);

        for word in words_iter {
            let characters_current = get_character_frequencies(word.clone());
            if characters_current != characters_last {
                result.push(word);
                characters_last = characters_current;
            }
        }

        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
