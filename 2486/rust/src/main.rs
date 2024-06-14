impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (mut first, mut longest_prefix) = (0, 0);
        while first < s.len() && longest_prefix < t.len() {
            if s.as_bytes()[first] == t.as_bytes()[longest_prefix] {
                longest_prefix += 1
            }
            first += 1
        }
        (t.len() - longest_prefix) as i32
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
