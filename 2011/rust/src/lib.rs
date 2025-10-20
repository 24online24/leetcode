impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.into_iter().fold(0, |acc, operation| {
            if operation.as_bytes()[1] == b'+' {
                acc + 1
            } else {
                acc - 1
            }
        })
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
