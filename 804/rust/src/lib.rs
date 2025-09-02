use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let conversion_table = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        words
            .into_iter()
            .map(|word| {
                word.bytes()
                    .map(|byte| conversion_table[(byte - b'a') as usize])
                    .collect::<String>()
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            Solution::unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ])
        );
    }
}
