impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        g.sort_unstable();
        s.sort_unstable();
        while j < g.len() {
            while i < s.len() && s[i] < g[j] {
                i += 1;
            }
            if i == s.len() {
                break;
            }
            if s[i] >= g[j] {
                i += 1;
                j += 1;
            }
        }
        j as i32
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
