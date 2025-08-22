impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut squares = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            for j in 0..n {
                let mut k = 0;
                let mut good = true;
                while good && i + k < m && j + k < n {
                    for row in matrix.iter().skip(i).take(k + 1) {
                        for element in row.iter().skip(j).take(k + 1) {
                            if *element == 0 {
                                good = false;
                                break;
                            }
                        }
                        if !good {
                            break;
                        }
                    }
                    if good {
                        k += 1;
                    }
                }
                squares += k;
            }
        }
        squares as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            7,
            Solution::count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
        )
    }
}
