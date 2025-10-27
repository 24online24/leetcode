impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits: Vec<_> = s.bytes().map(|b| b - b'0').collect();

        while digits.len() > 2 {
            let mut aux = vec![];
            for i in 0..digits.len() - 1 {
                aux.push((digits[i] + digits[i + 1]) % 10);
            }
            digits = aux;
        }

        digits[0] == digits[1]
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
