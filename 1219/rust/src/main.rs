use std::{cmp, vec};

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut max_gold = 0;

        for row in 0..rows {
            for col in 0..cols {
                max_gold = cmp::max(
                    max_gold,
                    Solution::dfs_backtrack(&mut grid, rows, cols, row as i32, col as i32),
                )
            }
        }
        max_gold
    }

    const DIRECTIONS: [i32; 5] = [0, 1, 0, -1, 0];

    fn dfs_backtrack(
        grid: &mut Vec<Vec<i32>>,
        rows: usize,
        cols: usize,
        row_int: i32,
        col_int: i32,
    ) -> i32 {
        if row_int < 0 || col_int < 0 {
            return 0;
        }
        let row = row_int as usize;
        let col = col_int as usize;
        if row == rows || col == cols || grid[row][col] == 0 {
            return 0;
        }
        let mut max_gold = 0;

        let original_val = grid[row][col];
        grid[row][col] = 0;

        (0..4).for_each(|direction| {
            max_gold = cmp::max(
                max_gold,
                Solution::dfs_backtrack(
                    grid,
                    rows,
                    cols,
                    Solution::DIRECTIONS[direction] + row as i32,
                    Solution::DIRECTIONS[direction + 1] + col as i32,
                ),
            )
        });

        grid[row][col] = original_val;
        max_gold + original_val
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::get_maximum_gold(vec![]));
}
