impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len() - 1;
        let mut result = digits;

        while i > 0 && result[i] == 9 {
            result[i] = 0;
            i -= 1;
        }
        if result[i] == 9 {
            result.push(0);
            result[i] = 1;
        } else {
            result[i] += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
