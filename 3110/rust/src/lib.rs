impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score = 0;
        let char_codes: Vec<i32> = s.chars().map(|c| c as i32).collect();
        for i in 1..char_codes.len() {
            score += (char_codes[i] - char_codes[i - 1]).abs()
        }
        score
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
