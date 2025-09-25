impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        fn compare_version_arrays(shorter: Vec<i32>, longer: Vec<i32>) -> i32 {
            let mut i = 0;
            while i < shorter.len() {
                if shorter[i] < longer[i] {
                    return -1;
                }
                if shorter[i] > longer[i] {
                    return 1;
                }
                i += 1;
            }
            while i < longer.len() {
                if longer[i] > 0 {
                    return -1;
                }
                i += 1;
            }
            0
        }

        let v1: Vec<i32> = version1.split('.').map(|r| r.parse().unwrap()).collect();
        let v2: Vec<i32> = version2.split('.').map(|r| r.parse().unwrap()).collect();

        if v1.len() <= v2.len() {
            compare_version_arrays(v1, v2)
        } else {
            -compare_version_arrays(v2, v1)
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            -1,
            Solution::compare_version("1.2".to_string(), "1.10".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::compare_version("1.01".to_string(), "1.001".to_string())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::compare_version("1.0".to_string(), "1.0.0.0".to_string())
        );
    }
}
