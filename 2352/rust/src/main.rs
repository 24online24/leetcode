use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut columns = HashMap::<Vec<i32>, i32>::new();
        let mut pairs = 0;
        let n = grid.len();
        for j in 0..n {
            let mut column = Vec::new();
            for i in 0..n {
                column.push(grid[i][j]);
            }
            *columns.entry(column).or_insert(0) += 1;
        }
        for row in grid {
            if let Some(&x) = columns.get(&row) {
                pairs += x;
            }
        }
        pairs
    }
}

struct Solution;

fn main() {
    let grid1 = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
    let grid2 = vec![
        vec![3, 1, 2, 2],
        vec![1, 4, 4, 5],
        vec![2, 4, 2, 2],
        vec![2, 4, 2, 2],
    ];
    println!("{}", Solution::equal_pairs(grid1));
    println!("{}", Solution::equal_pairs(grid2));
    println!("Hello, world!");
}
