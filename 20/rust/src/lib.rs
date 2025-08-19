impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parentheses = vec![];
        for chr in s.chars() {
            match chr {
                '(' => parentheses.push(chr),
                '[' => parentheses.push(chr),
                '{' => parentheses.push(chr),
                ')' => {
                    let last = parentheses.pop();
                    match last {
                        Some(parenthesis) => {
                            if parenthesis != '(' {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                ']' => {
                    let last = parentheses.pop();
                    match last {
                        Some(parenthesis) => {
                            if parenthesis != '[' {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                '}' => {
                    let last = parentheses.pop();
                    match last {
                        Some(parenthesis) => {
                            if parenthesis != '{' {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                _ => (),
            };
        }
        parentheses.is_empty()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::is_valid("()".to_string()));
        assert!(Solution::is_valid("()[]{}".to_string()));
        assert!(!Solution::is_valid("(]".to_string()));
        assert!(Solution::is_valid("([])".to_string()));
        assert!(!Solution::is_valid("([)]".to_string()));
    }
}
