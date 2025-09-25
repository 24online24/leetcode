impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() < 2 {
            return triangle[0][0];
        }
        for i in (0..=triangle.len() - 2).rev() {
            for j in 0..triangle[i].len() {
                triangle[i][j] += i32::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
            }
        }
        triangle[0][0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            11,
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(-10, Solution::minimum_total(vec![vec![-10]]));
    }
}
