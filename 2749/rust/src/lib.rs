impl Solution {
    pub fn make_the_integer_zero(mut num1: i32, num2: i32) -> i32 {
        let mut count = 0;
        while let Some(i) = (num1 - num2).checked_ilog2() && num1 > 0 {
            num1 = num1 - (2_i32.pow(i) + num2);
            count += 1;
        }
        if num1 == 0 {
            return count;
        }
        -1
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
