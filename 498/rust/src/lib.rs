impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut up = true;
        let m = mat.len();
        let n = mat[0].len();
        let mut i = 0;
        let mut j = 0;
        let mut array = Vec::with_capacity(m * n);
        while i < m - 1 || j < n - 1 {
            array.push(mat[i][j]);
            if up {
                if j == n - 1 {
                    i += 1;
                    up = false;
                } else if i == 0 {
                    j += 1;
                    up = false;
                } else {
                    i -= 1;
                    j += 1;
                }
            } else {
                if i == m - 1 {
                    j += 1;
                    up = true;
                } else if j == 0 {
                    i += 1;
                    up = true;
                } else {
                    i += 1;
                    j -= 1
                }
            }
        }
        array.push(mat[i][j]);
        array
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
    }
}
