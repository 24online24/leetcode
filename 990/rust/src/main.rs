impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {}
}

struct Solution;
fn main() {
    assert_eq!(
        Solution::equations_possible(vec!["a==b".to_owned(), "b!=a".to_owned()]),
        false
    );
    assert_eq!(
        Solution::equations_possible(vec!["b==a".to_owned(), "a==b".to_owned()]),
        true
    );
    println!("Hello, world!");
}
