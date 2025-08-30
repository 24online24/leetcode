use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            let mut appearances = HashSet::new();
            for j in 0..9 {
                if board[row][j] != '.' && !appearances.insert(&board[row][j]) {
                    return false;
                }
            }
        }
        for column in 0..9 {
            let mut appearances = HashSet::new();
            for i in 0..9 {
                if board[i][column] != '.' && !appearances.insert(&board[i][column]) {
                    return false;
                }
            }
        }
        for sub_box in 0..9 {
            let mut appearances = HashSet::new();
            for k in 0..9 {
                if board[3 * (sub_box / 3) + k / 3][3 * (sub_box % 3) + k % 3] != '.' && !appearances.insert(&board[3 * (sub_box / 3) + k / 3][3 * (sub_box % 3) + k % 3]) {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::is_valid_sudoku(vec![
            vec!["5","3",".",".","7",".",".",".","."],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ]));
    }
}
