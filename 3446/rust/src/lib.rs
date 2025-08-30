use std::mem::swap;

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let diag_count = 2 * grid.len() - 1;
        for diag in 0..diag_count {
            let mut n = (diag + 1).min(diag_count - diag);
            while n > 1{
                for i in 0..n {
                    swap(x, y);
                }
            }
        }
        grid
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]],
            Solution::sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]])
        );
    }
}
