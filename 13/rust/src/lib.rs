use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let NUMERALS: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut num = 0;
        let mut last = 0;
        let mut sum = 0;
        for char in s.chars() {
            let new = *NUMERALS.get(&char).unwrap();
            if new < last {
                num += sum;
                sum = 0;
            } else if new == last {
                sum += new;
            } else {
                sum = -sum;
                sum += new;
            }
            last = new;
        }
        num + sum
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
