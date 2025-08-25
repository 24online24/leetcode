impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + 2 * t
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
